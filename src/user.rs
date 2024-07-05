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
    pub username: String,
    pub path: Option<String>, // the user's location
    pub point: Option<isize>,
    pub region_beg: Option<isize>,
    pub region_end: Option<isize>,
    // Color definition
    pub color_cursor: Option<String>, // hex
    pub color_region: Option<String>, // hex
}

impl User {
    pub fn new(_username: String) -> Self {
        Self {
            username: _username,
            path: None,
            point: None,
            region_beg: None,
            region_end: None,
            color_cursor: None,
            color_region: None,
        }
    }

    pub fn update(
        &mut self,
        path: Option<String>,
        point: Option<isize>,
        region_beg: Option<isize>,
        region_end: Option<isize>,
        color_cursor: Option<String>,
        color_region: Option<String>,
    ) {
        self.path = path;
        self.point = point;
        self.region_beg = region_beg;
        self.region_end = region_end;
        self.color_cursor = color_cursor;
        self.color_region = color_region;
    }

    /// Move point information by delta.
    ///
    /// # Arguments
    ///
    /// * `_point` - Only effect point after the origin.
    /// * `_delta` - Movement delta.
    pub fn move_by_delta(&mut self, _point: isize, _delta: isize) {
        // Point must exists.
        if self.point.is_none() {
            return;
        }

        let point = self.point.unwrap();

        // Shift the point.
        if _point <= point {
            self.point = Some(point + _delta);

            // `region_beg`. and `region_end` must exists at the same time.
            if !self.region_beg.is_none() {
                let region_beg = self.region_beg.unwrap();
                let region_end = self.region_end.unwrap();

                // Shift the region.
                self.region_beg = Some(region_beg + _delta);
                self.region_end = Some(region_end + _delta);
            }
        }
    }
}
