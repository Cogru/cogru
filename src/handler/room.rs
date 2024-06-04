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

/// Enter room
pub mod enter {
    use crate::channel::*;
    use crate::room::*;
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

        let username = json["username"].as_str().unwrap().to_string();
        let password = if json["password"].is_null() {
            None
        } else {
            Some(json["password"].as_str().unwrap().to_string())
        };

        let (entered, message) = room.enter(addr, &username, &password);

        if entered {
            // Update client info!
            {
                let client = room.get_client_mut(addr).unwrap();
                client.enter_room(Some(username.clone()));
            }

            channel.broadcast_json(&serde_json::json!({
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

        let username = client.username().unwrap();

        channel.broadcast_json(&serde_json::json!({
            "method": METHOD,
            "message": format!("{} has left the room", username),
            "username": username,
            "status": "success",
        }));
    }
}

/// Send a room message.
///
/// This message goes across the project.
pub mod broadcast {
    use crate::channel::*;
    use crate::room::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "room::broadcast";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;
        let client = room.get_client_mut(addr).unwrap();

        if !client.entered() {
            channel
                .send_json(&serde_json::json!({
                    "method": METHOD,
                    "message": "You haven't entered the room yet",
                    "status": "failure",
                }))
                .await;
            return;
        }

        let message = json["message"].as_str().unwrap().to_string();

        channel.broadcast_json(&serde_json::json!({
            "method": METHOD,
            "username: ": client.username().unwrap(),
            "message": message,
            "status": "success",
        }));
    }
}

/// Room Users
///
/// Return a list of users in room.
pub mod users {
    use crate::channel::*;
    use crate::room::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        // TODO: ..
    }
}
