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
use crate::file::*;

pub struct Room {
    password: Option<String>,
    path: String,
    files: Vec<File>,
}

impl Room {
    pub fn new(_path: &str, _password: Option<String>) -> Self {
        Self {
            path: _path.to_string(),
            password: _password,
            files: Vec::new()
        }
    }

    /// Return true when room has password
    pub fn has_password(&self) -> bool {
        self.password != None
    }
}
