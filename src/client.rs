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
use crate::user::*;

#[derive(PartialEq)]
pub struct Client {
    entered: bool,      // Is inside the room?
    path: String,       // workspace path
    admin: bool,        // admin privileges
    user: Option<User>, // user view
}

impl Client {
    pub fn new(_path: String, _admin: bool) -> Self {
        Self {
            entered: false,
            path: _path,
            admin: _admin,
            user: None,
        }
    }

    /// Return true if this client is an admin.
    pub fn admin(&self) -> bool {
        self.admin
    }

    /// Return the user of this client.
    pub fn user(&self) -> Option<&User> {
        self.user.as_ref()
    }

    /// Return the user of this client. (mutable)
    pub fn user_mut(&mut self) -> Option<&mut User> {
        self.user.as_mut()
    }

    /// Return the user relative path.
    pub fn user_relative_path(&self) -> Option<String> {
        let user = self.user.clone();
        if user.is_none() {
            return None;
        }
        let path = user.unwrap().path();
        if path.is_none() {
            return None;
        }
        let path = path.unwrap();
        Some(path.replace(&self.path, ""))
    }

    /// Return project path
    pub fn get_path(&self) -> &String {
        &self.path
    }

    /// Return true if client has entered the room.
    pub fn entered(&self) -> bool {
        self.entered
    }

    /// Make client enter the room.
    ///
    /// # Arguments
    ///
    /// * `username` - Username of this client.
    pub fn enter_room(&mut self, username: &String) {
        self.user = Some(User::new(username.clone()));
        self.entered = true;
    }

    /// Make client leave the room.
    pub fn exit_room(&mut self) {
        self.user = None;
        self.entered = false;
    }
}
