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
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn check_opened(channel: &mut Channel, client: &mut Client, method: &str) {
    // TODO: ..
}

/// Convert path (absolute) to server local path.
///
/// # Arguments
///
/// * `addr` - Socket address used to get the client's project path.
/// * `room` - Used to get client and room path.
/// * `path` - Path we want to convert.
pub fn to_room_path(addr: &SocketAddr, room: &mut Room, path: &String) -> String {
    let server_path = room.get_path().clone();
    let client = room.get_client_mut(addr).unwrap();
    let project_path = client.get_path();
    path.replace(project_path, &server_path)
}

/// Open file
pub mod open {
    use crate::channel::*;
    use crate::handler::file::*;
    use crate::handler::room::*;
    use crate::room::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "file::open";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;

        // XXX: Get this early to avoid borrow errors.
        let file_path = json["file"].as_str().unwrap().to_string();
        let path = to_room_path(addr, &mut room, &file_path);

        let client = room.get_client_mut(addr).unwrap();
        let username = client.username().unwrap();

        if !check_entered(channel, client, METHOD).await {
            return;
        }

        // Get the registered file.
        let file = room.get_file(&path);

        // If not registered?
        if file.is_none() {
            // TODO: Create new file!
        }

        let file = file.unwrap();
        file.add_user(&username);
    }
}

/// Close file
pub mod close {
    use crate::channel::*;
    use crate::handler::room::*;
    use crate::room::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "file::close";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        // TODO: ..
    }
}

/// Sync file
pub mod sync {
    use crate::channel::*;
    use crate::handler::room::*;
    use crate::room::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "file::sync";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;
        let client = room.get_client_mut(addr).unwrap();

        if !check_entered(channel, client, METHOD).await {
            return;
        }

        let project_path = json["path"].as_str().unwrap().to_string();

        let file_path = json["file"].as_str().unwrap().to_string();

        // TODO: ..
    }
}

/// Say
pub mod say {
    use crate::channel::*;
    use crate::handler::room::*;
    use crate::room::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "file::say";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;
        let client = room.get_client_mut(addr).unwrap();

        if !check_entered(channel, client, METHOD).await {
            return;
        }

        let message = json["message"].as_str().unwrap().to_string();

        room.broadcast_json(&serde_json::json!({
            "method": METHOD,
            "message": message,
        }));
    }
}
