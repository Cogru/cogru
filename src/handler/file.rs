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

/// Return a list of users in the file.
pub mod users {
    use crate::channel::*;
    use crate::handler::room::*;
    use crate::room::*;
    use crate::util::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "file::users";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let room = room.lock().await;
        let client = room.get_client(addr).unwrap();

        if !check_entered(channel, &client, METHOD).await {
            return;
        }

        let this_user = client.user().unwrap();

        // If user is not in the file, ignore it.
        if this_user.path.is_none() {
            return;
        }

        // Prepare data to send.
        let mut users = Vec::new();

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
        let room = room.lock().await;
        let client = room.get_client(addr).unwrap();

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
