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
use crate::channel::*;
use crate::client::*;
use crate::room::*;
use crate::user::*;
use path_slash::PathBufExt as _;
use serde_json::Value;
use std::net::SocketAddr;
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

/// Get data as u64.
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

/// Get data as usize.
///
/// # Arguments
///
/// * `json` - JSON object.
/// * `key` - Key to the data.
pub fn data_usize(json: &Value, key: &str) -> Option<usize> {
    if json[key].is_null() {
        return None;
    }
    Some(json[key].to_string().parse::<usize>().unwrap())
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

/// Wrapper to fs::read_to_string
///
/// # Arguments
///
/// * `path` - File path to read.
pub fn read_to_string(path: &String) -> String {
    std::fs::read_to_string(path).expect(format!("Unable to read file: {}", path).as_str())
}

/// Convert path's absolute project path to this room path.
///
/// # Arguments
///
/// * `addr` - Socket address used to get the client's project path.
/// * `room` - Used to get client and room path.
/// * `path` - Path we want to convert.
pub fn to_room_path(addr: &SocketAddr, room: &Room, path: &str) -> String {
    let room_path = room.get_path().clone();
    let client = room.get_client(addr).unwrap();
    let project_path = client.get_path();
    path.replace(project_path, &room_path)
}

/// Remove room path.
///
/// # Arguments
///
/// * `room` - Room object.
/// * `path` - Path we want to remove room path.
pub fn no_room_path(room: &Room, path: &str) -> String {
    let room_path = room.get_path().clone();
    path.replace(&room_path, "")
}

/// Remove client path.
///
/// # Arguments
///
/// * `room` - Room object.
/// * `path` - Path we want to remove room path.
pub fn no_client_path(client: &Client, path: &Option<String>) -> Option<String> {
    if path.is_none() {
        return None;
    }
    let path = path.clone().unwrap();
    Some(path.replace(client.get_path(), ""))
}

/// Convert backslash to slash.
///
/// # Arguments
///
/// * `path` - Target path to be converted.
pub fn to_slash(path: &String) -> String {
    PathBuf::from_slash(path).to_slash().unwrap().to_string()
}
