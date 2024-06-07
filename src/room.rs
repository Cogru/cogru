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
use crate::chat::*;
use crate::client::*;
use crate::connection::*;
use crate::file::*;
use crate::file::*;
use ignore::WalkBuilder;
use serde_json::Value;
use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::fs::metadata;
use std::net::SocketAddr;
use std::path::Path;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

const COGUREIGNORE: &str = ".cogruignore";

type Tx = UnboundedSender<String>;
type Rx = UnboundedReceiver<String>;

pub struct Room {
    password: Option<String>, // room password
    pub peers: HashMap<SocketAddr, Tx>,
    path: String,                         // workspace path
    clients: HashMap<SocketAddr, Client>, // Connections in this room
    files: HashMap<String, File>,         // files are being visited
    chat: Chat,                           // messages in this file
}

impl Room {
    pub fn new(_path: &str, _password: Option<String>) -> Self {
        let mut room = Self {
            path: _path.to_string(),
            peers: HashMap::new(),
            password: _password,
            clients: HashMap::new(),
            files: HashMap::new(),
            chat: Chat::new(),
        };
        room.sync_files();
        room
    }

    /// Sync files in the room
    pub fn sync_files(&mut self) {
        let mut builder = WalkBuilder::new(&self.path);
        let ignore = self.ignore_file();

        builder.hidden(false); // make ignore files seeable

        // add custom ignore file.
        builder.add_custom_ignore_filename(ignore);

        for result in builder.build() {
            let dent = result.unwrap();
            let path = dent.path();
            let md = metadata(path).unwrap();

            if md.is_file() {
                let path = dent.path().display().to_string();
                println!("- {}", path);

                let file = File::new(path.clone());
                self.files.insert(path, file);
            }
        }
    }

    /// Return the project path.
    pub fn get_path(&self) -> &String {
        &self.path
    }

    /// Return a list of files need to be sync.
    pub fn get_files(&self) -> Vec<&String> {
        self.files.keys().clone().collect::<Vec<&String>>()
    }

    /// Return the custom ignore file path.
    fn ignore_file(&self) -> String {
        let ignore = Path::new(&self.path).join(COGUREIGNORE);
        String::from(ignore.to_str().unwrap())
    }

    /// Return true when room has password
    fn has_password(&self) -> bool {
        self.password != None
    }

    /// Return true if the username has already taken.
    ///
    /// # Arguments
    ///
    /// * `username` - The identifier in the room.
    fn username_taken(&self, addr: &SocketAddr, username: &String) -> bool {
        for (_addr, _client) in self.clients.iter() {
            // ignore ourselves
            if addr == _addr {
                continue;
            }

            // ignore client that haven't entered the room
            if !_client.entered() {
                continue;
            }

            if _client.username().unwrap() == *username {
                return true;
            }
        }
        return false;
    }

    /// Enter the room.
    ///
    /// # Arguments
    ///
    /// * `username` - The identifier in the room.
    /// * `password` - Check if the password is correct.
    pub fn enter(
        &self,
        addr: &SocketAddr,
        username: &String,
        password: &Option<String>,
    ) -> (bool, &str) {
        if self.username_taken(addr, username) {
            return (false, "Username already taken");
        }

        if !self.has_password() {
            return (true, "");
        }

        if password.is_none() {
            return (false, "Password cannot be null");
        }

        let password = password.as_ref().unwrap();

        if self.password.clone().unwrap() != *password {
            return (false, "Incorrect password");
        }

        return (true, "");
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

    /// Remove a client by address.
    ///
    /// # Arguments
    ///
    /// * `addr` - Key socket address.
    pub fn remove_client(&mut self, addr: &SocketAddr) {
        self.clients.remove(addr);
    }

    /// Return the socket address by username.
    ///
    /// # Arguments
    ///
    /// * `username` - The client username.
    pub fn get_client_by_name(&mut self, username: &str) -> Option<&mut Client> {
        for (addr, client) in self.clients.iter_mut() {
            if client.username().unwrap() == username {
                return Some(client);
            }
        }
        None
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

    /// Click a client.
    ///
    /// # Arguments
    ///
    /// * `username` - The client username.
    pub fn kick(&mut self, username: &str) -> (bool, String) {
        let client = self.get_client_by_name(username);

        if client.is_none() {
            return (false, format!("User `{}` not found in the room", username));
        }

        let client = client.unwrap();

        if !client.entered() {
            return (false, format!("User `{}` is not in the room", username));
        }

        client.exit_room();
        return (true, "".to_string());
    }
}
