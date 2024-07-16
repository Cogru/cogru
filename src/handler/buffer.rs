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
pub use std::net::SocketAddr;

/// Addition and Deletion to the buffer.
pub mod update {
    use crate::handler::buffer::*;

    const METHOD: &str = "buffer::update";

    fn predict_movement(addr: &SocketAddr, room: &mut Room, point: isize, delta: isize) {
        let client = room.get_client_mut(addr).unwrap();

        let filename = client.move_self(point);

        let clients = room.get_clients_mut();

        for (_addr, _client) in clients.iter_mut() {
            if _addr == addr {
                continue;
            }

            _client.move_other(point, delta, filename.clone());
        }
    }

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;

        let path = data_str(json, "path").unwrap();
        let add_or_delete = data_str(json, "add_or_delete").unwrap();
        let beg = data_isize(json, "beg").unwrap();
        let end = data_isize(json, "end").unwrap();
        let contents = data_str(json, "contents").unwrap();

        // Predict movement.
        {
            let is_delete = add_or_delete == "delete";

            // Predict the movement shift.
            let delta = if is_delete { beg - end } else { end - beg };

            // Predict the cursor final position.
            let start = if is_delete { beg } else { end };

            // Nothing has changed; return it.
            if delta == 0 {
                return;
            }

            predict_movement(&addr, &mut room, start, delta);
        }

        // Update the buffer view.
        let file = room.get_file_create_mut(&addr, &path, None);
        let file = file.unwrap();

        let rel_file = file.relative_path();
        let rel_file = file.relative_path();

        file.update(&add_or_delete, beg, end, &contents);

        let client = room.get_client(addr).unwrap();

        // Get the peers that are in the file.
        let peers = room.peers_by_file(&room, &rel_file);

        let params = &serde_json::json!({
            "method": METHOD,
            "username": client.user().unwrap().username,
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

/// Save the buffer.
pub mod save {
    use crate::handler::buffer::*;

    const METHOD: &str = "buffer::save";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr.clone();
        let mut room = room.lock().await;
        let client = room.get_client(addr).unwrap();

        if !check_entered(channel, client, METHOD).await {
            return;
        }

        let filename = data_str(json, "file");
        let contents = data_str(json, "contents");

        if filename.is_none() {
            missing_field(channel, METHOD, "file").await;
            return;
        }

        if contents.is_none() {
            missing_field(channel, METHOD, "contents").await;
            return;
        }

        let rel_filename = no_client_path(&client, &filename);

        let filename = filename.unwrap();

        if rel_filename.is_none() {
            general_error(
                channel,
                METHOD,
                format!("The file is not under the project path: {}", filename),
            )
            .await;
            return;
        }

        let file = room.get_file_create_mut(addr, &filename, contents);
        let file = file.unwrap();

        file.save();

        let rel_filename = rel_filename.unwrap();
        let contents = file.buffer();

        room.broadcast_json_except(
            &serde_json::json!({
                "method": METHOD,
                "file": rel_filename,
                "contents": contents,
                "status": ST_SUCCESS,
            }),
            addr,
        );
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
