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
    use crate::handler::file::*;
    use crate::handler::room::*;
    use crate::room::*;
    use crate::util::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "file::update";

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
    use crate::handler::file::*;
    use crate::handler::room::*;
    use crate::room::*;
    use crate::util::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "file::save";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;

        let path = data_str(json, "path").unwrap();
        let file = room.get_file_mut(&addr, &path);

        if file.is_none() {
            tracing::debug!("Updating an non-existence file: {}", path);
            // TODO: Create one?
            return;
        }

        let file = file.unwrap();
        file.save();

        let contents = file.contents();
        let relative_path = no_room_path(&room, &path);

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
        let room = room.lock().await;

        // XXX: Get this early to avoid borrow errors.
        let file_path = data_str(json, "file").unwrap();
        let local_path = to_room_path(addr, &room, &file_path);

        let client = room.get_client(addr).unwrap();

        // Check entered the room.
        if !check_entered(channel, client, METHOD).await {
            return;
        }

        let contents = read_to_string(&local_path);

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

/// Return a list of users in the file.
pub mod info {
    use crate::channel::*;
    use crate::client::*;
    use crate::handler::room::*;
    use crate::room::*;
    use crate::user::*;
    use crate::util::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "file::info";

    /// Return a list of user in file.
    ///
    /// # Arguments
    ///
    /// * `room` - It's used to get all users in room.
    /// * `client` - Need the target client's file path.
    fn get_users(room: &Room, client: &Client) -> Vec<User> {
        // Prepare data to send.
        let mut users = Vec::new();

        let this_user = client.user().unwrap();

        // If user is not in the file, ignore it.
        if this_user.path.is_none() {
            return users;
        }

        for _client in room.get_clients().iter() {
            let user = _client.user();

            if user.is_none() {
                continue;
            }

            let user = user.unwrap();

            // Ignore the sender client.
            if this_user == user {
                continue;
            }

            // Ignore when user not visiting any project files.
            if user.path.is_none() {
                continue;
            }

            // Ignore if not in the same file.
            if client.user_relative_path() != _client.user_relative_path() {
                continue;
            }

            users.push(user.clone());
        }

        users
    }

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let room = room.lock().await;
        let client = room.get_client(addr).unwrap();

        if !check_entered(channel, &client, METHOD).await {
            return;
        }

        let users = get_users(&room, &client);
        let users = serde_json::to_string(&users).unwrap();

        channel
            .send_json(&serde_json::json!({
                "method": METHOD,
                "clients": users,
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
        let room = room.lock().await;
        let client = room.get_client(addr).unwrap();

        if !check_entered(channel, client, METHOD).await {
            return;
        }

        let username = client.user().unwrap().username.clone();

        let file = data_str(json, "file").unwrap();
        let file = no_room_path(&room, &file);
        let message = data_str(json, "message").unwrap();

        // Get the peers that are in the file.
        let peers = room.peers_by_file(&room, &file);

        let params = &serde_json::json!({
            "method": METHOD,
            "username": username,  // Who speak this message?
            "file": file,
            "message": message,
            "status": "success",
        });

        for (_addr, _sender) in peers.iter() {
            let _ = _sender.send(params.to_string());
        }
    }
}
