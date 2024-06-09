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
use path_slash::PathBufExt as _;
use serde_json::Value;
use std::path::PathBuf;

/// Get data as string.
///
/// # Arguments
///
/// * `json` - JSON object.
/// * `key` - Key to the data.
pub fn data_str(json: &Value, key: &str) -> String {
    json[key].as_str().unwrap().to_string()
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
