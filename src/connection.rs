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
use crate::handler;
use async_recursion::async_recursion;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const SEPARATOR_LEN: usize = "\r\n".len();
const BUF_SIZE: usize = 1024 * 1;

pub struct Connection {
    pub stream: tokio::net::TcpStream,
    pub addr: std::net::SocketAddr,
    read_buf: [u8; BUF_SIZE],
    data: Vec<u8>,
    pub entered: bool,
}

impl Connection {
    pub fn new(_stream: tokio::net::TcpStream, _addr: std::net::SocketAddr) -> Self {
        let connection = Self {
            stream: _stream,
            addr: _addr,
            read_buf: [0; BUF_SIZE],
            data: Vec::new(),
            entered: false,
        };
        connection
    }

    /// Logic loop.
    pub async fn run(&mut self) {
        loop {
            self.read().await;
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

    /// Raw data receiver.
    pub async fn read(&mut self) {
        let _ = match self.stream.read(&mut self.read_buf).await {
            // socket closed
            Ok(n) if n == 0 => return,
            Ok(n) => {
                tracing::trace!("{} ({:?})", self.to_string(), n);

                // Add new data to the end of data buffer.
                {
                    let new_data = &self.read_buf[0..n];
                    self.data.append(&mut new_data.to_vec());
                }

                self.process().await;

                n
            }
            Err(e) => {
                println!("Failed to read from socket; err = {:?}", e);
                return;
            }
        };
    }

    /// Process through the request if available.
    #[async_recursion]
    pub async fn process(&mut self) {
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
                    content_len = Connection::get_content_len(line);
                }
                1 => {
                    boundary += line.len() + SEPARATOR_LEN;
                }
                2 => {
                    if content_len <= line.len() {
                        boundary += content_len;

                        let data = &line[..content_len];
                        handler::handle(self, data).await;
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
            self.process().await;
        }
    }

    /// Write the raw data through the tunnel.
    ///
    /// # Arguments
    ///
    /// * `buf` - The buffer; vector of bytes.
    async fn write(&mut self, buf: &[u8]) {
        if let Err(e) = self.stream.write_all(&buf).await {
            tracing::warn!("Failed to write to socket {:?}; err = {:?}", self.stream, e);
            return;
        }
    }

    /// Send the CSP JSON request.
    ///
    /// # Arguments
    ///
    /// * `params` - JSON object.
    pub async fn send(&mut self, params: serde_json::Value) {
        let json_str = params.to_string();
        let data_str = format!("Content-Length: {}\r\n\r\n{}", json_str.len(), json_str);
        let data = data_str.as_bytes();
        self.write(&data).await;
    }

    /// Return the connection string.
    pub fn to_string(&self) -> String {
        format!("{}", &self.addr)
    }
}
