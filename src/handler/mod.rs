/**
 * Copyright (c) 2024-2025 Cogru Inc.
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
pub mod buffer;
pub mod file;
pub mod room;
pub mod util;

pub use crate::channel::*;
pub use crate::client::*;
pub use crate::constant::*;
pub use crate::room::*;
pub use crate::util::*;
pub use serde_json::Value;
pub use std::sync::Arc;
pub use tokio::sync::Mutex;

pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &str) {
    let v = serde_json::from_str(json);
    let val: Value = v.unwrap();
    let method: &str = val["method"].as_str().unwrap();

    match method {
        "test" => test::handle(channel, room, &val).await,
        "init" => init::handle(channel, room, &val).await,
        "ping" => ping::handle(channel, room, &val).await,
        "room::enter" => room::enter::handle(channel, room, &val).await,
        "room::exit" => room::exit::handle(channel, room, &val).await,
        "room::add_file" => room::add_file::handle(channel, room, &val).await,
        "room::delete_file" => room::delete_file::handle(channel, room, &val).await,
        "room::rename_file" => room::rename_file::handle(channel, room, &val).await,
        "room::kick" => room::kick::handle(channel, room, &val).await,
        "room::broadcast" => room::broadcast::handle(channel, room, &val).await,
        "room::info" => room::info::handle(channel, room, &val).await,
        "room::sync" => room::sync::handle(channel, room, &val).await,
        "room::update_client" => room::update_client::handle(channel, room, &val).await,
        "room::find_user" => room::find_user::handle(channel, room, &val).await,
        "file::sync" => file::sync::handle(channel, room, &val).await,
        "file::info" => file::info::handle(channel, room, &val).await,
        "file::say" => file::say::handle(channel, room, &val).await,
        "buffer::update" => buffer::update::handle(channel, room, &val).await,
        "buffer::sync" => buffer::sync::handle(channel, room, &val).await,
        "buffer::save" => buffer::save::handle(channel, room, &val).await,
        _ => {
            tracing::error!("Unkown method request: `{}`", method);
        }
    }
}

/// Test
mod test {
    use crate::handler::*;

    const METHOD: &str = "test";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        tracing::trace!("method: {:?}", json["method"]);

        let room = room.lock().await;

        channel
            .send_json(&serde_json::json!({
                "method": METHOD,
                "some": "ラウトは難しいです！",
            }))
            .await;

        room.broadcast_json(&serde_json::json!({
            "method": METHOD,
            "message": "This is the broadcast test!",
        }));
    }
}

/// Ping pong
mod ping {
    use crate::handler::*;
    use chrono;

    const METHOD: &str = "pong";

    pub async fn handle(channel: &mut Channel, _room: &Arc<Mutex<Room>>, _json: &Value) {
        channel
            .send_json(&serde_json::json!({
                "method": METHOD,
                "timestamp": chrono::offset::Local::now().to_string(),
                "status": ST_SUCCESS,
            }))
            .await;
    }
}

/// Initialize for client that has first connected.
mod init {
    use crate::handler::*;

    const METHOD: &str = "init";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let mut room = room.lock().await;

        let path = data_str(json, "path").unwrap();

        // XXX: Every local client is the admin.
        let is_admin = channel.is_local(&room);
        let client = Client::new(path.clone(), is_admin);

        room.add_client(channel.get_connection().addr, client);

        channel
            .send_json(&serde_json::json!({
                "method": METHOD,
                "message": format!("Done initialized in [{}]", path),
                "status": ST_SUCCESS,
            }))
            .await;
    }
}
