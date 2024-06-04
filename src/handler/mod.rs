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
pub mod room;

use crate::channel::*;
use crate::handler::room::*;
use crate::room::*;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &str) {
    let v = serde_json::from_str(json);
    let val: Value = v.unwrap();
    let method: &str = val["method"].as_str().unwrap();

    match method {
        "test" => test::handle(channel, room, &val).await,
        "ping" => ping::handle(channel, room, &val).await,
        "enter" => enter::handle(channel, room, &val).await,
        "exit" => exit::handle(channel, room, &val).await,
        "broadcast" => broadcast::handle(channel, room, &val).await,
        _ => {
            tracing::error!("Unkown method request: {:?}", method);
        }
    }
}

/// Test
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
            "method": "test",
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
