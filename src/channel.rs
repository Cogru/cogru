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
use crate::handler;
use crate::room::*;
use async_recursion::async_recursion;
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::broadcast::Sender;
use tokio::sync::Mutex;

const SEPARATOR_LEN: usize = "\r\n".len();
const BUF_SIZE: usize = 1024 * 1;

pub struct Channel {
    read_buf: [u8; BUF_SIZE],
    data: Vec<u8>,
    tx: Sender<String>,
    connection: Connection,
}

impl Channel {
    pub fn new(_connection: Connection, _tx: Sender<String>) -> Self {
        Self {
            read_buf: [0; BUF_SIZE],
            data: Vec::new(),
            tx: _tx,
            connection: _connection,
        }
    }

    /// Return true when channel is local.
    pub fn is_local(&self) -> bool {
        let ip = self.connection.addr.ip();
        println!("add: {:?}", ip.to_string());
        ip.to_string() == "127.0.0.1"
    }

    /// Logic loop.
    pub async fn run(&mut self, room: &Arc<Mutex<Room>>) {
        let mut rx = self.tx.subscribe();

        // Start receiving messages.
        loop {
            tokio::select! {
                result = self.connection.stream.read(&mut self.read_buf) => {
                    let n = match result {
                        Err(e) => {
                            tracing::error!("Reading data: {}", e);
                            return;
                        }
                        Ok(n) if n == 0 => {
                            self.disconnect(room).await;
                            return;
                        }
                        Ok(n) => n,
                    };

                    tracing::trace!("{} ({:?})", self.connection.addr, n);

                    // Add new data to the end of data buffer.
                    {
                        let new_data = &self.read_buf[0..n];
                        self.data.append(&mut new_data.to_vec());
                    }

                    self.process(room).await;
                }
                msg = rx.recv() => {
                    if let Ok(data) = msg {
                        //stream.write(data.as_bytes()).await.unwrap();
                        self.connection.send_json_str(&data).await;
                    }
                }
            }
        }
    }

    /// Process through the request if available.
    #[async_recursion]
    async fn process(&mut self, room: &Arc<Mutex<Room>>) {
        let data = &self.data.clone();
        let decrypted = String::from_utf8_lossy(data);

        let chopped = decrypted.split("\r\n");
        let size = chopped.clone().count();

        if size < 3 {
            return;
        }

        let mut content_len: usize = 0;
        let mut op = 0;
        let mut boundary = 0;
        let mut process = false;

        for line in chopped {
            let current_op = op % 3;

            match current_op {
                0 => {
                    boundary += line.len() + SEPARATOR_LEN;
                    content_len = get_content_len(line);
                }
                1 => {
                    boundary += line.len() + SEPARATOR_LEN;
                }
                2 => {
                    if content_len <= line.len() {
                        boundary += content_len;

                        let data = &line[..content_len];
                        handler::handle(self, room, data).await;
                        //println!("{}: {}", "receive all", data);

                        process = true;
                        break;
                    }
                }
                _ => {
                    tracing::error!("Invalid operation id: {:?}", current_op);
                }
            }
            op += 1;
        }

        if process {
            self.data = self.data[boundary..].to_vec();
            tracing::trace!(
                "data left ({}) {:?}",
                boundary,
                String::from_utf8_lossy(&self.data)
            );
            self.process(room).await;
        }
    }

    pub async fn disconnect(&self, room: &Arc<Mutex<Room>>) {
        tracing::trace!("{} disconnected", self.connection.addr);

        let mut room = room.lock().await;

        room.remove_client(&self.connection.addr);
    }

    pub fn get_connection(&mut self) -> &mut Connection {
        &mut self.connection
    }

    pub fn get_stream(&mut self) -> &mut TcpStream {
        &mut self.connection.stream
    }

    /// Wrapper for function Connection::send_json_str
    pub async fn send_json_str(&mut self, json_str: &String) {
        self.get_connection().send_json_str(json_str).await;
    }

    /// Wrapper for function Connection::send_json
    pub async fn send_json(&mut self, params: &Value) {
        self.get_connection().send_json(params).await;
    }

    /// Send JSON data to all clients.
    ///
    /// # Arguments
    ///
    /// * `params` - [description]
    pub fn broadcast_json(&self, params: &Value) {
        let _ = self.tx.send(params.to_string());
    }
}

/// Return the contnet length.
///
/// # Arguments
///
/// * `line` - The line string.
fn get_content_len(line: &str) -> usize {
    if !line.starts_with("Content-Length: ") {
        tracing::error!("Invalid content length: {:?}", line);
        return 0;
    }
    let rm_len = "Content-Length: ".len();
    let len_str = &line[rm_len..];
    len_str.parse::<usize>().unwrap()
}
