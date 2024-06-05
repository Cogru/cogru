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
use crate::chat::*;

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

pub struct File {
    path: String,     // absolute path
    users: Vec<User>, // a list of users in the file
    chat: Chat,       // messages in this file
    content: String,  // the file content
}

impl File {
    pub fn new(_path: String) -> Self {
        Self {
            path: _path,
            users: Vec::new(),
            chat: Chat::new(),
            content: String::default(),
        }
    }
}
