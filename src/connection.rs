/**
 * Copyright (c) 2024-2025 Cogru Inc.
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
use serde_json::Value;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct Connection {
    pub stream: TcpStream,
    pub addr: SocketAddr,
}

impl Connection {
    pub fn new(_stream: TcpStream, _addr: SocketAddr) -> Self {
        Self {
            stream: _stream,
            addr: _addr,
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

    /// Send the JSON string.
    ///
    /// # Arguments
    ///
    /// * `params` - JSON object.
    pub async fn send_json_str(&mut self, json_str: &String) {
        let data_str = format!("Content-Length: {}\r\n\r\n{}", json_str.len(), json_str);
        let data = data_str.as_bytes();
        self.write(&data).await;
    }

    /// Send the CSP JSON request.
    ///
    /// # Arguments
    ///
    /// * `params` - JSON object.
    pub async fn send_json(&mut self, params: &Value) {
        let json_str = params.to_string();
        self.send_json_str(&json_str).await;
    }

    /// Return the connection string.
    pub fn to_string(&self) -> String {
        format!("{}", &self.addr)
    }
}
