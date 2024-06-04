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
use crate::client::*;

struct Region {
    start: u64,
    end: u64,
}

struct Mouse {
    point: u64,
}

struct User {
    username: String,
    mouse: Mouse,
    region: Region,
}

struct Message {
    username: String,
    content: String,
    timestamp: String,
}

impl Message {
    pub fn new(client: &Client, _content: &str) -> Self {
        Self {
            // XXX: When create message, the username cannot be dangling.
            username: client.username().unwrap(),
            content: _content.to_string(),
            timestamp: chrono::offset::Local::now().to_string(),
        }
    }
}

pub struct File {
    path: String,           // absolute path
    users: Vec<User>,       // a list of users in the file
    messages: Vec<Message>, // messages in this file
    content: String,        // the file content
}
