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
pub use crate::channel::*;
pub use crate::client::*;
pub use crate::constant::*;
pub use crate::handler::util::*;
pub use crate::room::*;
pub use crate::server::error::*;
pub use crate::user::*;
pub use crate::util::*;
pub use serde_json::Value;
pub use std::sync::Arc;
pub use tokio::sync::Mutex;

/// Enter room
pub mod enter {
    use crate::handler::room::*;

    const METHOD: &str = "room::enter";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;
        let client = room.get_client_mut(addr).unwrap();

        if client.entered() {
            channel
                .send_json(&serde_json::json!({
                    "method": METHOD,
                    "message": "You have already entered the room",
                    "status": ST_FAILURE,
                }))
                .await;
            return;
        }

        let username = data_str(json, "username").unwrap();
        let password = data_str(json, "password");

        let (entered, message) = room.enter(addr, &username, &password);

        if entered {
            // Update client info!
            {
                let client = room.get_client_mut(addr).unwrap();
                client.enter_room(&username);
            }

            room.broadcast_json(&serde_json::json!({
                "method": METHOD,
                "message": format!("{} has entered the room", username),
                "username": username,
                "status": ST_SUCCESS,
            }));
        } else {
            channel
                .send_json(&serde_json::json!({
                    "method": METHOD,
                    "message": message,
                    "status": ST_FAILURE,
                }))
                .await;
        }
    }
}

/// Enter room
pub mod exit {
    use crate::handler::room::*;

    const METHOD: &str = "room::exit";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, _json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;
        let client = room.get_client_mut(addr).unwrap();

        if !client.entered() {
            channel
                .send_json(&serde_json::json!({
                    "method": METHOD,
                    "message": "You never entered the room; do nothing",
                    "status": ST_FAILURE,
                }))
                .await;
            return;
        }

        let user = client.user();
        let username = user.unwrap().username.clone();

        // Leave the room
        client.exit_room();

        room.broadcast_json(&serde_json::json!({
            "method": METHOD,
            "message": format!("{} has left the room", username),
            "username": username,
            "status": ST_SUCCESS,
        }));
    }
}

/// Add a new file.
pub mod add_file {
    use crate::handler::room::*;

    const METHOD: &str = "room::add_file";

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

        // If already exists, return it.
        {
            let file = room.get_file(addr, &filename);

            if !file.is_none() {
                return;
            }
        }

        // Create it!
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

/// Delete a file.
pub mod delete_file {
    use crate::handler::room::*;

    const METHOD: &str = "room::delete_file";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr.clone();
        let mut room = room.lock().await;
        let client = room.get_client(addr).unwrap();

        if !check_entered(channel, client, METHOD).await {
            return;
        }

        let filename = data_str(json, "file");

        if filename.is_none() {
            missing_field(channel, METHOD, "file").await;
            return;
        }

        let rel_filename = no_client_path(&client, &filename);

        let filename = filename.unwrap();

        let abs_filename = to_room_path(&addr, &room, &filename);

        let file = room.delete_file(&abs_filename);

        // Failed to delete file.
        if file.is_none() {
            general_error(
                channel,
                METHOD,
                format!("Fail to delete file, doesn't exists: {}", filename),
            )
            .await;
            return;
        }

        let rel_filename = rel_filename.unwrap();

        room.broadcast_json_except(
            &serde_json::json!({
                "method": METHOD,
                "file": rel_filename,
                "status": ST_SUCCESS,
            }),
            addr,
        );
    }
}

/// Rename a file.
pub mod rename_file {
    use crate::handler::room::*;

    const METHOD: &str = "room::rename_file";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr.clone();
        let mut room = room.lock().await;
        let client = room.get_client(addr).unwrap();

        if !check_entered(channel, client, METHOD).await {
            return;
        }

        let filename = data_str(json, "file");
        let newname = data_str(json, "newname");

        let rel_filename = no_client_path(&client, &filename);
        let rel_newname = no_client_path(&client, &newname);

        if filename.is_none() {
            missing_field(channel, METHOD, "file").await;
            return;
        }

        if newname.is_none() {
            missing_field(channel, METHOD, "newname").await;
            return;
        }

        let rel_filename = no_client_path(&client, &filename);
        let rel_newname = no_client_path(&client, &newname);

        let filename = filename.unwrap();
        let newname = newname.unwrap();

        let abs_filename = to_room_path(&addr, &room, &filename);
        let abs_newname = to_room_path(&addr, &room, &newname);

        let file = room.rename_file(&abs_filename, &abs_newname);

        // Failed to rename file.
        if file.is_none() {
            general_error(
                channel,
                METHOD,
                format!("Fail to rename file, doesn't exists: {}", filename),
            )
            .await;
            return;
        }

        let rel_filename = rel_filename.unwrap();
        let rel_newname = rel_newname.unwrap();

        room.broadcast_json_except(
            &serde_json::json!({
                "method": METHOD,
                "file": rel_filename,
                "newname": rel_newname,
                "status": ST_SUCCESS,
            }),
            addr,
        );
    }
}

/// Kcik the user out of the room.
pub mod kick {
    use crate::handler::room::*;

    const METHOD: &str = "room::kick";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;
        let client = room.get_client(addr).unwrap();

        if !check_entered(channel, client, METHOD).await {
            return;
        }

        // Only the admin privileges can kick the user out!
        if !check_admin(channel, &client, METHOD).await {
            return;
        }

        let admin_name = client.user().unwrap().username.clone();
        // target user to kick out
        let target_name = data_str(json, "username").unwrap();

        // kick
        let (kicked, message) = room.kick(&target_name);

        if kicked {
            room.broadcast_json(&serde_json::json!({
                "method": METHOD,
                "username": target_name,
                "admin": admin_name,
                "message": format!("{} has been kicked out by {}", target_name, admin_name),
                "status": ST_SUCCESS,
            }));
            return;
        }

        channel
            .send_json(&serde_json::json!({
                "method": METHOD,
                "username": target_name,
                "message": message,
                "status": ST_FAILURE,
            }))
            .await;
    }
}

/// Send a room message.
///
/// This message goes across the project.
pub mod broadcast {
    use crate::handler::room::*;

    const METHOD: &str = "room::broadcast";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let room = room.lock().await;
        let client = room.get_client(addr).unwrap();

        let username = client.user().unwrap().username.clone();

        if !check_entered(channel, client, METHOD).await {
            return;
        }

        let message = data_str(json, "message").unwrap();

        room.broadcast_json(&serde_json::json!({
            "method": METHOD,
            "username": username,
            "message": message,
            "status": ST_SUCCESS,
        }));
    }
}

/// Update a single client's information.
pub mod update_client {
    use crate::handler::room::*;

    const METHOD: &str = "room::update_client";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr.clone();
        let mut room = room.lock().await;

        {
            let client = room.get_client_mut(addr).unwrap();

            if !check_entered(channel, client, METHOD).await {
                return;
            }
        }

        let path = data_str(json, "path");
        let point = data_isize(json, "point");
        let region_beg = data_isize(json, "region_beg");
        let region_end = data_isize(json, "region_end");
        let color_cursor = data_str(json, "color_cursor");
        let color_region = data_str(json, "color_region");

        let client = room.get_client(addr).unwrap();
        let rel_path = no_client_path(&client, &path);

        let client = room.get_client_mut(addr).unwrap();
        let user = client.user_mut().unwrap();

        user.update(
            rel_path,
            point,
            region_beg,
            region_end,
            color_cursor,
            color_region,
        );
    }
}

/// Room Information
///
/// Return a list of users in room.
pub mod info {
    use crate::handler::room::*;

    const METHOD: &str = "room::info";

    fn get_users(room: &Room) -> Vec<User> {
        let mut users = Vec::new();

        for client in room.get_clients_vec().iter() {
            let user = client.user();

            // User not entered yet.
            if user.is_none() {
                continue;
            }

            users.push(user.unwrap().clone());
        }

        users
    }

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let room = room.lock().await;
        let client = room.get_client(addr).unwrap();

        if !check_entered(channel, client, METHOD).await {
            return;
        }

        let users = get_users(&room);
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

/// Sync the entire room.
pub mod sync {
    use crate::handler::room::*;

    const METHOD: &str = "room::sync";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let room = room.lock().await;
        let client = room.get_client(addr).unwrap();

        if !check_entered(channel, client, METHOD).await {
            return;
        }

        let project_path = data_str(json, "path").unwrap();

        let room_path = room.get_path().clone();
        let files = room.get_path_files();

        for file in files.into_iter() {
            let abs_path = file;
            let contents = read_to_string(&abs_path);

            // Replace the room path to client's project path, so the client
            // can use the path directly.
            let file_path = abs_path.replace(&room_path, &project_path);
            let file_path = to_slash(&file_path);

            channel
                .send_json(&serde_json::json!({
                    "method": METHOD,
                    "file": file_path,
                    "contents": contents,
                    "status": ST_SUCCESS,
                }))
                .await;
        }
    }
}

/// Return user's position.
pub mod find_user {
    use crate::handler::room::*;

    const METHOD: &str = "room::find_user";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let room = room.lock().await;

        let username = data_str(json, "username");

        if username.is_none() {
            missing_field(channel, METHOD, "username").await;
            return;
        }

        let username = username.unwrap();

        let client = room.get_client_by_name(&username);

        if client.is_none() {
            general_error(channel, METHOD, "Client not found in the room").await;
            return;
        }

        let client = client.unwrap();

        let user = client.user().unwrap();

        let path = user.path.clone().unwrap();
        let point = user.point.unwrap();

        channel
            .send_json(&serde_json::json!({
                "method": METHOD,
                "username": username,
                "file": path,
                "point": point,
                "status": ST_SUCCESS,
            }))
            .await;
    }
}
