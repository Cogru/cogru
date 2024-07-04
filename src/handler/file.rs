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
pub use crate::channel::*;
pub use crate::constant::*;
pub use crate::handler::room::*;
pub use crate::room::*;
pub use crate::user::*;
pub use crate::util::*;
pub use serde_json::Value;
pub use std::sync::Arc;
pub use tokio::sync::Mutex;

/// Sync file
///
/// Replace the client's file with server file; sync file
/// can make user's lose his work!
pub mod sync {
    use crate::handler::file::*;

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

        // Get file string, not the buffer!
        let contents = read_to_string(&local_path);

        channel
            .send_json(&serde_json::json!({
                "method": METHOD,
                "file": file_path,  // send it back directly
                "contents": contents,
                "status": ST_SUCCESS,
            }))
            .await;
    }
}

/// Return a list of users in the file.
pub mod info {
    use crate::handler::file::*;

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

        for _client in room.get_clients_vec().iter() {
            let user = _client.user();

            // User not entered yet.
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
                "status": ST_SUCCESS,
            }))
            .await;
    }
}

/// Say
pub mod say {
    use crate::handler::file::*;

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
            "status": ST_SUCCESS,
        });

        for (_addr, _sender) in peers.iter() {
            let _ = _sender.send(params.to_string());
        }
    }
}

/// Lock the file.
///
/// Only user who locked the file and admins can edit the file.
pub mod lock {
    use crate::handler::file::*;

    const METHOD: &str = "file::lock";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let file = data_str(json, "file");

        if file.is_none() {
            return;
        }

        // TODO: ..
    }
}
