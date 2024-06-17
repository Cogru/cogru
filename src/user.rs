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

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    username: String,
    path: Option<String>, // the user's location
    point: Option<u64>,
    region_beg: Option<u64>,
    region_end: Option<u64>,
}

impl User {
    pub fn new(_username: String) -> Self {
        Self {
            username: _username,
            path: None,
            point: None,
            region_beg: None,
            region_end: None,
        }
    }

    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn path(&self) -> Option<String> {
        self.path.clone()
    }

    pub fn point(&self) -> Option<u64> {
        self.point
    }

    pub fn region_beg(&self) -> Option<u64> {
        self.region_beg
    }

    pub fn region_end(&self) -> Option<u64> {
        self.region_end
    }

    pub fn update(
        &mut self,
        path: Option<String>,
        point: Option<u64>,
        region_beg: Option<u64>,
        region_end: Option<u64>,
    ) {
        self.path = path;
        self.point = point;
        self.region_beg = region_beg;
        self.region_end = region_end;
    }
}
