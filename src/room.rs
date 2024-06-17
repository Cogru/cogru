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
use crate::server::properties::*;
use crate::util::*;
use ignore::WalkBuilder;
use serde_json::Value;
use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::fs::metadata;
use std::net::SocketAddr;
use std::path::Path;
use tokio::sync::mpsc::UnboundedSender;

const COGUREIGNORE: &str = ".cogruignore";

pub struct Room {
    prop: Properties,                                    // server properties
    password: Option<String>,                            // room password
    path: String,                                        // workspace path
    peers: HashMap<SocketAddr, UnboundedSender<String>>, // broadcasting
    clients: HashMap<SocketAddr, Client>,                // Connections in this room
    files: HashMap<String, File>,                        // files are being visited
    chat: Chat,                                          // messages in this file
}

impl Room {
    pub fn new(_prop: Properties, _path: &str, _password: Option<String>) -> Self {
        let mut room = Self {
            prop: _prop,
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

    /// Return properties object.
    pub fn get_prop(&self) -> &Properties {
        &self.prop
    }

    /// Get peers
    pub fn peers(&mut self) -> &mut HashMap<SocketAddr, UnboundedSender<String>> {
        &mut self.peers
    }

    /// Return peers by file.
    ///
    /// # Arguments
    ///
    /// * `file` - The file path.
    pub fn peers_by_file(
        &self,
        room: &Room,
        file: &String,
    ) -> HashMap<&SocketAddr, &UnboundedSender<String>> {
        let file = no_room_path(&room, &file);

        let mut data = HashMap::new();

        for (addr, sender) in self.peers.iter() {
            let client = self.get_client(addr).unwrap();
            let path = no_room_path(&room, client.get_path());
            if path == file {
                continue;
            }
            data.insert(addr, sender);
        }

        data
    }

    /// Return the sender.
    ///
    /// # Arguments
    ///
    /// * `addr` - Socket Address.
    pub fn get_sender(&mut self, addr: &SocketAddr) -> &mut UnboundedSender<String> {
        self.peers.get_mut(addr).unwrap()
    }

    /// Send JSON object.
    ///
    /// # Arguments
    ///
    /// * `addr` - Socket Address.
    /// * `params` - JSON object.
    pub fn send_json(&mut self, addr: &SocketAddr, params: &Value) {
        let sender = self.get_sender(addr);
        let _ = sender.send(params.to_string());
    }

    /// Send JSON data to all clients.
    ///
    /// # Arguments
    ///
    /// * `params` - [description]
    pub fn broadcast_json(&self, params: &Value) {
        for (_addr, sender) in self.peers.iter() {
            let _ = sender.send(params.to_string());
        }
    }

    /// Send JSON data to all clients.
    ///
    /// # Arguments
    ///
    /// * `params` - [description]
    pub fn broadcast_json_except(&self, params: &Value, addr: &SocketAddr) {
        for (_addr, sender) in self.peers.iter() {
            if _addr == addr {
                continue;
            }
            let _ = sender.send(params.to_string());
        }
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
                let path = to_slash(&path);
                println!("  - Sync file {}", path);

                let file = File::new(path.clone());
                self.files.insert(path, file);
            }
        }
    }

    /// Return the project path.
    pub fn get_path(&self) -> &String {
        &self.path
    }

    /// Return the file object by file path.
    ///
    /// # Arguments
    ///
    /// * `SocketAddr` - Socket address to convert to full path.
    /// * `path` - The file path.
    pub fn get_file(&self, addr: &SocketAddr, path: &String) -> Option<&File> {
        let path = to_room_path(addr, self, path);
        self.files.get(&path)
    }

    /// Return the file object by file path.
    ///
    /// # Arguments
    ///
    /// * `SocketAddr` - Socket address to convert to full path.
    /// * `path` - The file path.
    pub fn get_file_mut(&mut self, addr: &SocketAddr, path: &String) -> Option<&mut File> {
        let path = to_room_path(addr, self, path);
        self.files.get_mut(&path)
    }

    /// Return a list of files need to be sync.
    pub fn get_path_files(&self) -> Vec<&String> {
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

            if _client.user().unwrap().username() == *username {
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

    /// Return a list of client.
    pub fn get_clients(&self) -> Vec<&Client> {
        self.clients.values().collect::<Vec<&Client>>()
    }

    /// Return a list of client.
    pub fn get_clients_mut(&mut self) -> Vec<&mut Client> {
        self.clients.values_mut().collect::<Vec<&mut Client>>()
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

    /// Remove a client by address.
    ///
    /// # Arguments
    ///
    /// * `addr` - Key socket address.
    pub fn remove_peer(&mut self, addr: &SocketAddr) {
        self.peers.remove(addr);
    }

    /// Return the socket address by username.
    ///
    /// # Arguments
    ///
    /// * `username` - The client username.
    pub fn get_client_by_name(&self, username: &str) -> Option<&Client> {
        for (addr, client) in self.clients.iter() {
            if client.user().unwrap().username() == username {
                return Some(client);
            }
        }
        None
    }

    /// Return the socket address by username.
    ///
    /// # Arguments
    ///
    /// * `username` - The client username.
    pub fn get_client_mut_by_name(&mut self, username: &str) -> Option<&mut Client> {
        for (addr, client) in self.clients.iter_mut() {
            if client.user().unwrap().username() == username {
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
        let client = self.get_client_mut_by_name(username);

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
