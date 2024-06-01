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
use crate::connection::*;
use crate::room::*;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle(channel: &Channel, room: &mut Room, json: &str) {
    let v = serde_json::from_str(json);
    let val: serde_json::Value = v.unwrap();

    println!("{}: {}", "method", val["method"]);

    let method: &str = val["method"].as_str().unwrap();
    println!("{}: {:?}", "val", val["method"]);

    //let room = room.lock().await;

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
            // TODO: ..
        }
        _ => {
            tracing::error!("Unkown method request: {:?}", method);
        }
    }
}

mod test {
    use crate::channel::*;
    use crate::client::*;
    use crate::connection::*;
    use crate::room::*;

    pub async fn handle(channel: &Channel, room: &mut Room, json: &serde_json::Value) {
        tracing::trace!("method: {:?}", json["method"]);

        let client = room.get_client_mut(&channel.addr).unwrap();

        client
            .get_connection()
            .send(&serde_json::json!({
                "method": "test",
                "some": "ラウトは難しいです！",
            }))
            .await;

        room.broadcast(&serde_json::json!({
            "method": "test_broadcast",
            "message": "This is the broadcast test!",
        }))
        .await;
    }
}

/// Ping pong
mod ping {
    use crate::channel::*;
    use crate::client::*;
    use crate::connection::*;
    use crate::room::*;
    use chrono;

    pub async fn handle(channel: &Channel, room: &mut Room, json: &serde_json::Value) {
        let client = room.get_client_mut(&channel.addr).unwrap();

        client
            .get_connection()
            .send(&serde_json::json!({
                "method": "pong",
                "timestamp": chrono::offset::Local::now().to_string(),
            }))
            .await;
    }
}

/// Enter session
mod enter {
    use crate::channel::*;
    use crate::client::*;
    use crate::connection::*;
    use crate::room::*;

    pub async fn handle(channel: &Channel, room: &mut Room, json: &serde_json::Value) {
        let username = json["username"].clone().to_string();
        let password = json["password"].clone().to_string();

        let client = room.get_client_mut(&channel.addr).unwrap();

        let entered: bool = false;
        {
            // let room = client.get_room().lock().await;
            // entered = room.enter(username, password);
            //
            // if entered {
            //     room.add_client(client);
            // }
        }

        if entered {
            //client.entered = true;

            client
                .get_connection()
                .send(&serde_json::json!({
                    "method": "enter",
                    "message": "Successully entered the room",
                }))
                .await;
        } else {
            client
                .get_connection()
                .send(&serde_json::json!({
                    "method": "enter",
                    "message": "Incorrect password",
                }))
                .await;
        }
    }
}
