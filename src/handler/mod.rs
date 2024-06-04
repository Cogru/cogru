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
use crate::room::*;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &str) {
    let v = serde_json::from_str(json);
    let val: Value = v.unwrap();
    let method: &str = val["method"].as_str().unwrap();

    match method {
        "test" => {
            test::handle(channel, room, &val).await;
        }
        "ping" => {
            ping::handle(channel, room, &val).await;
        }
        "enter" => {
            enter::handle(channel, room, &val).await;
        }
        "exit" => {
            exit::handle(channel, room, &val).await;
        }
        _ => {
            tracing::error!("Unkown method request: {:?}", method);
        }
    }
}

mod test {
    use crate::channel::*;
    use crate::room::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    pub async fn handle(channel: &mut Channel, _room: &Arc<Mutex<Room>>, json: &Value) {
        tracing::trace!("method: {:?}", json["method"]);

        //let mut room = room.lock().await;
        //let client = room.get_client_mut(&channel.addr).unwrap();

        channel
            .send_json(&serde_json::json!({
                "method": "test",
                "some": "ラウトは難しいです！",
            }))
            .await;

        channel.broadcast_json(&serde_json::json!({
            "method": "test_broadcast",
            "message": "This is the broadcast test!",
        }));
    }
}

/// Ping pong
mod ping {
    use crate::channel::*;
    use crate::room::*;
    use chrono;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    pub async fn handle(channel: &mut Channel, _room: &Arc<Mutex<Room>>, _json: &Value) {
        channel
            .send_json(&serde_json::json!({
                "method": "pong",
                "timestamp": chrono::offset::Local::now().to_string(),
            }))
            .await;
    }
}

/// Enter room
mod enter {
    use crate::channel::*;
    use crate::room::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;
        let client = room.get_client_mut(addr).unwrap();

        if client.entered() {
            channel
                .send_json(&serde_json::json!({
                    "method": "enter",
                    "message": "You have already entered the room",
                    "status": "failure",
                }))
                .await;
            return;
        }

        let username = json["username"].to_string();
        let password = if json["password"].is_null() {
            None
        } else {
            Some(json["password"].to_string())
        };

        let (entered, message) = room.enter(addr, &username, &password);

        if entered {
            // Update client info!
            {
                let client = room.get_client_mut(addr).unwrap();
                client.enter_room(Some(username.clone()));
            }

            channel
                .send_json(&serde_json::json!({
                    "method": "enter",
                    "message": "You have successully entered the room",
                    "username": username,
                    "status": "success",
                }))
                .await;
        } else {
            channel
                .send_json(&serde_json::json!({
                    "method": "enter",
                    "message": message,
                    "status": "failure",
                }))
                .await;
        }
    }
}

/// Enter room
mod exit {
    use crate::channel::*;
    use crate::room::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, _json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;
        let client = room.get_client_mut(addr).unwrap();

        if !client.entered() {
            channel
                .send_json(&serde_json::json!({
                    "method": "exit",
                    "message": "You never entered the room; do nothing",
                    "status": "failure",
                }))
                .await;
            return;
        }

        // Leave the room
        client.exit_room();

        channel
            .send_json(&serde_json::json!({
                "method": "exit",
                "message": "You have successfully left the room",
                "status": "success",
            }))
            .await;
    }
}