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

type RegionBeg = u64;
type RegionEnd = u64;

pub struct User {
    pub username: String,
    pub path: Option<String>, // the user's location
    pub point: Option<u64>,
    pub region: Option<(RegionBeg, RegionEnd)>,
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

    pub fn update(
        &mut self,
        path: &String,
        point: &String,
        region_start: &String,
        region_end: &String,
    ) {
        self.path = Some(path.clone());
        // TODO: ..
        self.point = Some(0);
        self.region = Some((0, 0));
    }
}
