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
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn check_opened(channel: &mut Channel, client: &mut Client, method: &str) {
    // TODO: ..
}

/// Open file
pub mod open {
    use crate::channel::*;
    use crate::handler::room::*;
    use crate::room::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "file::open";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        // TODO: ..
    }
}

/// Close file
pub mod close {
    use crate::channel::*;
    use crate::handler::room::*;
    use crate::room::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "file::close";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        // TODO: ..
    }
}

/// Sync file
pub mod sync {
    use crate::channel::*;
    use crate::handler::room::*;
    use crate::room::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "file::sync";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        let addr = &channel.get_connection().addr;
        let mut room = room.lock().await;
        let client = room.get_client_mut(addr).unwrap();

        if !check_entered(channel, client, METHOD).await {
            return;
        }

        let project_path = json["path"].as_str().unwrap().to_string();

        let file_path = json["file"].as_str().unwrap().to_string();

        // TODO: ..
    }
}

/// Say
pub mod say {
    use crate::channel::*;
    use crate::handler::room::*;
    use crate::room::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const METHOD: &str = "file::say";

    pub async fn handle(channel: &mut Channel, room: &Arc<Mutex<Room>>, json: &Value) {
        if !ensure_entered(channel, room, METHOD).await {
            return;
        }

        let message = json["message"].as_str().unwrap().to_string();

        channel.broadcast_json(&serde_json::json!({
            "method": METHOD,
            "message": message,
        }));
    }
}
