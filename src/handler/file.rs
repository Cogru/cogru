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

/// Convert path's absolute project path to this room path.
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

/// Return a list of users in the file.
pub mod users {
    use crate::channel::*;
    use crate::handler::room::*;
    use crate::room::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "file::users";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        // TODO: ..
    }
}

/// Sync file
pub mod sync {
    use crate::channel::*;
    use crate::handler::file::*;
    use crate::handler::room::*;
    use crate::room::*;
    use crate::util::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "file::sync";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;

        // XXX: Get this early to avoid borrow errors.
        let file_path = data_str(json, "file").unwrap();
        let local_path = to_room_path(addr, &mut room, &file_path);

        let client = room.get_client_mut(addr).unwrap();

        // Check entered the room.
        if !check_entered(channel, client, METHOD).await {
            return;
        }

        let content = read_to_string(&local_path);

        channel
            .send_json(&serde_json::json!({
                "method": METHOD,
                "file": file_path,  // send it back directly
                "content": content,
                "status": "success",
            }))
            .await;
    }
}

/// Say
pub mod say {
    use crate::channel::*;
    use crate::handler::room::*;
    use crate::room::*;
    use crate::util::*;
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

        let message = data_str(json, "message").unwrap();

        room.broadcast_json(&serde_json::json!({
            "method": METHOD,
            "message": message,
        }));
    }
}
