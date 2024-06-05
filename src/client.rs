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
use crate::connection::*;
use crate::handler;
use crate::room::*;

pub struct Client {
    username: Option<String>, // name of this client
    entered: bool,            // Is inside the room?
    path: String,             // workspace path
    admin: bool,              // admin privileges
}

impl Client {
    pub fn new(_path: String, _admin: bool) -> Self {
        Self {
            username: None,
            entered: false,
            path: _path,
            admin: _admin,
        }
    }

    /// Return true if this client is an admin.
    pub fn admin(&self) -> bool {
        self.admin
    }

    /// Return the username of this client.
    pub fn username(&self) -> Option<String> {
        self.username.clone()
    }

    pub fn entered(&self) -> bool {
        self.entered
    }

    pub fn enter_room(&mut self, username: Option<String>) {
        self.username = username;
        self.entered = true;
    }

    pub fn exit_room(&mut self) {
        self.username = None;
        self.entered = false;
    }
}
