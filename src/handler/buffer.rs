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
pub use crate::handler::room::*;

/// Addition and Deletion to the buffer.
pub mod update {
    use crate::handler::buffer::*;

    const METHOD: &str = "buffer::update";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;

        let path = data_str(json, "path").unwrap();
        let add_or_delete = data_str(json, "add_or_delete").unwrap();
        let beg = data_usize(json, "beg").unwrap();
        let end = data_usize(json, "end").unwrap();
        let contents = data_str(json, "contents").unwrap();

        let file = room.get_file_create_mut(&addr, &path, None);
        let file = file.unwrap();

        let rel_file = file.relative_path();

        file.update(&add_or_delete, beg, end, &contents);

        // Get the peers that are in the file.
        let peers = room.peers_by_file(&room, &rel_file);

        let params = &serde_json::json!({
            "method": METHOD,
            "file": rel_file,
            "add_or_delete": add_or_delete,
            "beg": beg,
            "end": end,
            "contents": contents,
            "status": ST_SUCCESS,
        });

        for (_addr, _sender) in peers.iter() {
            if *_addr == addr {
                continue;
            }
            let _ = _sender.send(params.to_string());
        }
    }
}

/// Synce the buffer.
///
/// This will only sync the view.
pub mod sync {
    use crate::handler::buffer::*;

    const METHOD: &str = "buffer::sync";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr.clone();
        let mut room = room.lock().await;

        let client = room.get_client(addr).unwrap();

        // Check entered the room.
        if !check_entered(channel, client, METHOD).await {
            return;
        }

        let filename = data_str(json, "file");

        if filename.is_none() {
            missing_field(channel, METHOD, "file").await;
            return;
        }

        let filename = filename.unwrap();

        let file = room.get_file_mut(addr, &filename);

        if file.is_none() {
            tracing::debug!("Updating an non-existence file: {}", filename);
            // TODO: Create one?
            return;
        }

        let file = file.unwrap();

        let contents = file.buffer();

        channel
            .send_json(&serde_json::json!({
                "method": METHOD,
                "file": filename,  // send it back directly
                "contents": contents,
                "status": ST_SUCCESS,
            }))
            .await;
    }
}

/// Lock the file.
///
/// Only user who locked the file and admins can edit the file.
pub mod lock {
    use crate::handler::buffer::*;

    const METHOD: &str = "buffer::lock";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let file = data_str(json, "file");

        if file.is_none() {
            return;
        }

        // TODO: ..
    }
}
