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

/// Addition and Deletion to the file.
pub mod update {
    use crate::channel::*;
    use crate::room::*;
    use crate::util::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "buffer::update";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;

        let path = data_str(json, "path").unwrap();
        let add_or_delete = data_str(json, "add_or_delete").unwrap();
        let beg = data_usize(json, "beg").unwrap();
        let end = data_usize(json, "end").unwrap();
        let contents = data_str(json, "contents").unwrap();

        let file = room.get_file(&addr, &path);

        if file.is_none() {
            tracing::debug!("Updating an non-existence file: {}", path);
            // TODO: Create one?
            return;
        }

        let relative_path = file.unwrap().relative_path(&room);

        let file = room.get_file_mut(&addr, &path);
        let file = file.unwrap();

        file.update(&add_or_delete, beg, end, &contents);

        // Get the peers that are in the file.
        let peers = room.peers_by_file(&room, &relative_path);

        let params = &serde_json::json!({
            "method": METHOD,
            "file": relative_path,
            "add_or_delete": add_or_delete,
            "beg": beg,
            "end": end,
            "contents": contents,
            "status": "success",
        });

        for (_addr, _sender) in peers.iter() {
            if *_addr == addr {
                continue;
            }
            let _ = _sender.send(params.to_string());
        }
    }
}

/// Save file.
pub mod save {
    use crate::channel::*;
    use crate::room::*;
    use crate::util::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "buffer::save";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;
        let client = room.get_client(addr).unwrap();

        let path = data_str(json, "file");
        let relative_path = no_client_path(&client, &path);

        let path = path.unwrap();
        let file = room.get_file_mut(&addr, &path);

        if file.is_none() {
            tracing::debug!("Updating an non-existence file: {}", path);
            // TODO: Create one?
            return;
        }

        let file = file.unwrap();
        file.save();

        let contents = file.buffer();

        let relative_path = relative_path.unwrap();

        room.broadcast_json_except(
            &serde_json::json!({
                "method": METHOD,
                "file": relative_path,
                "contents": contents,
                "status": "success",
            }),
            addr,
        );
    }
}

/// Synce the buffer.
///
/// This will only sync the view.
pub mod sync {
    use crate::channel::*;
    use crate::client::*;
    use crate::handler::room::*;
    use crate::room::*;
    use crate::user::*;
    use crate::util::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "buffer::sync";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr.clone();
        let mut room = room.lock().await;

        let client = room.get_client(addr).unwrap();

        // Check entered the room.
        if !check_entered(channel, client, METHOD).await {
            return;
        }

        let file_path = data_str(json, "file").unwrap();

        let file = room.get_file_mut(addr, &file_path);

        // TODO: Handle `file` is none error.

        let file = file.unwrap();

        let contents = file.buffer();

        channel
            .send_json(&serde_json::json!({
                "method": METHOD,
                "file": file_path,  // send it back directly
                "contents": contents,
                "status": "success",
            }))
            .await;
    }
}

/// Lock the file.
///
/// Only user who locked the file and admins can edit the file.
pub mod lock {
    use crate::channel::*;
    use crate::room::*;
    use crate::util::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "buffer::lock";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let file = data_str(json, "file");

        if file.is_none() {
            return;
        }

        // TODO: ..
    }
}
