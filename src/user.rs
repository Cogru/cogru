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

#[derive(Default)]
pub struct Region {
    start: u64,
    end: u64,
}

#[derive(Default)]
pub struct Mouse {
    point: u64,
}

pub struct User {
    pub username: String,
    pub path: String, // the user's location
    pub mouse: Mouse,
    pub region: Region,
}

impl User {
    pub fn new(_username: String) -> Self {
        Self {
            username: _username,
            path: "".to_string(),
            mouse: Mouse::default(),
            region: Region::default(),
        }
    }
}
