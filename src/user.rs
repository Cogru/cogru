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
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Region {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

impl Region {
    pub fn new(_start: Option<u64>, _end: Option<u64>) -> Self {
        Self {
            start: _start,
            end: _end,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub path: Option<String>, // the user's location
    pub point: Option<u64>,
    pub region: Option<Region>,
}

impl User {
    pub fn new(_username: String) -> Self {
        Self {
            username: _username,
            path: None,
            point: None,
            region: None,
        }
    }

    pub fn update(&mut self, path: Option<String>, point: Option<u64>, region: Option<Region>) {
        self.path = path;
        self.point = point;
        self.region = region;
    }
}
