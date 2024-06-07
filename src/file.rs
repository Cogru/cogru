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
use crate::user::*;
use std::collections::HashMap;

pub struct File {
    path: String,                 // absolute path
    users: HashMap<String, User>, // a list of users in the file
    chat: Chat,                   // messages in this file
    view: String,                 // the file view
}

impl File {
    pub fn new(_path: String) -> Self {
        Self {
            path: _path,
            users: HashMap::new(),
            chat: Chat::new(),
            view: String::default(),
        }
    }

    /// Return the file path.
    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn add_user(&mut self, username: &String) {
        self.users.insert(username.clone(), User::new(username.clone()));
    }

    /// Write the content to file.
    pub async fn save(&self) {
        // TODO: ..
    }
}
