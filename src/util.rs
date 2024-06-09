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
use path_slash::PathBufExt as _;
use serde_json::Value;
use std::path::PathBuf;

/// Get data as string.
///
/// # Arguments
///
/// * `json` - JSON object.
/// * `key` - Key to the data.
pub fn data_str(json: &Value, key: &str) -> Option<String> {
    if json[key].is_null() {
        return None;
    }
    Some(json[key].as_str().unwrap().to_string())
}

/// Get data as string.
///
/// # Arguments
///
/// * `json` - JSON object.
/// * `key` - Key to the data.
pub fn data_u64(json: &Value, key: &str) -> Option<u64> {
    if json[key].is_null() {
        return None;
    }
    Some(json[key].as_u64().unwrap())
}

/// Parse data to u64.
///
/// # Arguments
///
/// * `data` - Target data to be parsed.
pub fn parse_u64(data: Option<String>) -> Option<u64> {
    if data.is_none() {
        return None;
    }
    let data = data.unwrap();
    let data = Some(data.parse::<u64>().unwrap());
    data
}

/// Convert two Option<64> to region tuple.
///
/// # Arguments
///
/// * `start` - Start region point.
/// * `end` - End region point.
pub fn to_region(start: Option<u64>, end: Option<u64>) -> Option<Region> {
    if start.is_none() || end.is_none() {
        None
    } else {
        Some(Region::new(start, end))
    }
}

/// Wrapper to fs::read_to_string
///
/// # Arguments
///
/// * `path` - File path to read.
pub fn read_to_string(path: &String) -> String {
    std::fs::read_to_string(path).expect(format!("Unable to read file: {}", path).as_str())
}

/// Convert backslash to slash.
///
/// # Arguments
///
/// * `path` - Target path to be converted.
pub fn to_slash(path: &String) -> String {
    PathBuf::from_slash(path).to_slash().unwrap().to_string()
}
