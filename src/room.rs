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
use crate::client::*;
use crate::connection::*;
use crate::file::*;
use serde_json::Value;
use std::collections::HashMap;
use std::net::SocketAddr;

pub struct Room {
    password: Option<String>,             // room password
    path: String,                         // workspace path
    files: Vec<File>,                     // files are being visited
    clients: HashMap<SocketAddr, Client>, // Connections in this room
}

impl Room {
    pub fn new(_path: &str, _password: Option<String>) -> Self {
        Self {
            path: _path.to_string(),
            password: _password,
            files: Vec::new(),
            clients: HashMap::new(),
        }
    }

    /// Return true when room has password
    fn has_password(&self) -> bool {
        self.password != None
    }

    /// Enter the room.
    ///
    /// # Arguments
    ///
    /// * `username` - The identifier in the room.
    /// * `password` - Check if the password is correct.
    pub fn enter(&self, username: String, password: String) -> bool {
        if !self.has_password() {
            return true;
        }

        return self.password.clone().unwrap() == password;
    }

    /// Add a client to room.
    ///
    /// # Arguments
    ///
    /// * `addr` - Socket address as key.
    /// * `client` - Target client.
    pub fn add_client(&mut self, addr: SocketAddr, client: Client) {
        self.clients.insert(addr, client);
    }

    /// Return the client as immutable.
    ///
    /// # Arguments
    ///
    /// * `addr` - Key socket address.
    pub fn get_client(&self, addr: &SocketAddr) -> Option<&Client> {
        self.clients.get(addr)
    }

    /// Return the client as mutable.
    ///
    /// # Arguments
    ///
    /// * `addr` - Key socket address.
    pub fn get_client_mut(&mut self, addr: &SocketAddr) -> Option<&mut Client> {
        self.clients.get_mut(addr)
    }

    ///  Send data to all clients in this room.
    pub async fn broadcast(&mut self, params: &Value) {
        for (addr, client) in self.clients.iter_mut() {
            client.get_connection().send_json(params).await;
        }
    }

    /// Broadcast params except addr.
    ///
    /// # Arguments
    ///
    /// * `addr` - The addr we do not want to send to.
    /// * `params` - data to send.
    pub async fn broadcast_excl(&mut self, addr: SocketAddr, params: &Value) {
        for (addr, client) in self.clients.iter_mut() {
            if addr == addr {
                continue;
            }
            client.get_connection().send_json(params).await;
        }
    }
}
