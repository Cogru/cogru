/**
 * Copyright (c) 2024 Cogru Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use crate::channel::*;
use crate::client::*;
use crate::room::*;
use serde_json::Value;
use std::fs;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Check if the client has entered the room.
///
/// Use this when re-using client and room variables.
///
/// # Arguments
///
/// * `channel` - Send error message when not entered.
/// * `client` - The client to check to see if entered.
/// * `method` - The method id.
pub async fn check_entered(channel: &mut Channel, client: &mut Client, method: &str) -> bool {
    if client.entered() {
        return true;
    }

    channel
        .send_json(&serde_json::json!({
            "method": method,
            "message": "You haven't entered the room yet",
            "status": "failure",
        }))
        .await;

    return false;
}

/// Ensure the client is entered.
///
/// Use this when not re-using client and room variables.
///
/// # Arguments
///
/// * `channel` - Send error message when not entered.
/// * `room` - Room use to get the client.
/// * `method` - The method id.
pub async fn ensure_entered(channel: &mut Channel, room: &Arc<Mutex<Room>>, method: &str) -> bool {
    let addr = &channel.get_connection().addr;
    let mut room = room.lock().await;
    let client = room.get_client_mut(addr).unwrap();

    check_entered(channel, client, method).await
}

/// Return true if admin; else false and send the error message to the client.
///
/// # Arguments
///
/// * `channel` - Used when sending the error message.
/// * `client` - Client to see if it has the admin privileges.
/// * `method` - The method id.
pub async fn check_admin(channel: &mut Channel, client: &mut Client, method: &str) -> bool {
    if client.admin() {
        return true;
    }

    channel
        .send_json(&serde_json::json!({
            "method": method,
            "message": "You are not the admin; only admin can operate this action",
            "status": "failure",
        }))
        .await;

    return false;
}

/// Enter room
pub mod enter {
    use crate::channel::*;
    use crate::room::*;
    use crate::util::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "room::enter";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;
        let client = room.get_client_mut(addr).unwrap();

        if client.entered() {
            channel
                .send_json(&serde_json::json!({
                    "method": METHOD,
                    "message": "You have already entered the room",
                    "status": "failure",
                }))
                .await;
            return;
        }

        let username = data_str(json, "username");
        let password = if json["password"].is_null() {
            None
        } else {
            Some(data_str(json, "password"))
        };

        let (entered, message) = room.enter(addr, &username, &password);

        if entered {
            // Update client info!
            {
                let client = room.get_client_mut(addr).unwrap();
                client.enter_room(&username);
            }

            room.broadcast_json(&serde_json::json!({
                "method": METHOD,
                "message": format!("{} has entered the room", username),
                "username": username,
                "status": "success",
            }));
        } else {
            channel
                .send_json(&serde_json::json!({
                    "method": METHOD,
                    "message": message,
                    "status": "failure",
                }))
                .await;
        }
    }
}

/// Enter room
pub mod exit {
    use crate::channel::*;
    use crate::room::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "room::exit";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, _json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;
        let client = room.get_client_mut(addr).unwrap();

        if !client.entered() {
            channel
                .send_json(&serde_json::json!({
                    "method": METHOD,
                    "message": "You never entered the room; do nothing",
                    "status": "failure",
                }))
                .await;
            return;
        }

        // Leave the room
        client.exit_room();

        let username = client.user().unwrap().username.clone();

        room.broadcast_json(&serde_json::json!({
            "method": METHOD,
            "message": format!("{} has left the room", username),
            "username": username,
            "status": "success",
        }));
    }
}

/// Kcik the user out of the room.
pub mod kick {
    use crate::channel::*;
    use crate::handler::room::*;
    use crate::room::*;
    use crate::util::*;
    use serde_json::Value;
    use std::sync::{Arc, MutexGuard};
    use tokio::sync::Mutex;

    const METHOD: &str = "room::kick";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;
        let client = room.get_client_mut(addr).unwrap();

        if !check_entered(channel, client, METHOD).await {
            return;
        }

        // Only the admin privileges can kick the user out!
        if !check_admin(channel, client, METHOD).await {
            return;
        }

        let admin_name = client.user().unwrap().username.clone();
        // target user to kick out
        let target_name = data_str(json, "username");

        // kick
        let (kicked, message) = room.kick(&target_name);

        if kicked {
            room.broadcast_json(&serde_json::json!({
                "method": METHOD,
                "username": target_name,
                "admin": admin_name,
                "message": format!("{} has been kicked out by {}", target_name, admin_name),
                "status": "success",
            }));
            return;
        }

        channel
            .send_json(&serde_json::json!({
                "method": METHOD,
                "username": target_name,
                "message": message,
                "status": "failure",
            }))
            .await;
    }
}

/// Send a room message.
///
/// This message goes across the project.
pub mod broadcast {
    use crate::channel::*;
    use crate::handler::room::*;
    use crate::room::*;
    use crate::util::*;
    use serde_json::Value;
    use std::sync::{Arc, MutexGuard};
    use tokio::sync::Mutex;

    const METHOD: &str = "room::broadcast";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;
        let client = room.get_client_mut(addr).unwrap();

        let username = client.user().unwrap().username.clone();

        if !check_entered(channel, client, METHOD).await {
            return;
        }

        let message = data_str(json, "message");

        room.broadcast_json(&serde_json::json!({
            "method": METHOD,
            "username": username,
            "message": message,
            "status": "success",
        }));
    }
}

/// Update a single client's information.
pub mod update {
    use crate::channel::*;
    use crate::handler::room::*;
    use crate::room::*;
    use crate::util::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "room::update";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;
        let client = room.get_client_mut(addr).unwrap();

        if !check_entered(channel, client, METHOD).await {
            return;
        }

        let path = data_str(json, "path");
        let point = data_str(json, "point");
        let region_start = data_str(json, "region_start");
        let region_end = data_str(json, "region_end");

        let user = client.user_mut().unwrap();

        user.update(&path, &point, &region_start, &region_end);
    }
}

/// Room Users
///
/// Return a list of users in room.
pub mod users {
    use crate::channel::*;
    use crate::handler::room::*;
    use crate::room::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "room::users";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;
        let client = room.get_client_mut(addr).unwrap();

        if !check_entered(channel, client, METHOD).await {
            return;
        }

        for client in room.get_clients().iter_mut() {
            let user = client.user_mut();

            // TODO: ..
        }
    }
}

/// Sync the entire room.
pub mod sync {
    use crate::channel::*;
    use crate::handler::room::*;
    use crate::room::*;
    use crate::util::*;
    use path_slash::PathBufExt as _;
    use serde_json::Value;
    use std::path::{Path, PathBuf};
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "room::sync";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;
        let client = room.get_client_mut(addr).unwrap();

        if !check_entered(channel, client, METHOD).await {
            return;
        }

        let project_path = data_str(json, "path");

        let room_path = room.get_path().clone();
        let files = room.get_files();

        for file in files.into_iter() {
            let abs_path = file;
            let content = read_to_string(&abs_path);

            // Replace the room path to client's project path, so the client
            // can use the path directly.
            let path = abs_path.replace(&room_path, &project_path);
            let path = PathBuf::from_slash(&path).to_slash().unwrap().to_string();

            channel
                .send_json(&serde_json::json!({
                    "method": METHOD,
                    "path": path,
                    "content": content,
                    "status": "success",
                }))
                .await;
        }
    }
}
