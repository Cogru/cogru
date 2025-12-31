/**
 * Copyright (c) 2024-2026 Cogru Inc.
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
pub mod error;
pub mod properties;
use crate::channel::*;
use crate::connection::*;
use crate::room::*;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

pub struct Server {
    host: String,
    port: u16,
    room: Arc<Mutex<Room>>,
}

impl Server {
    pub fn new(_host: &str, _port: u16, _room: Room) -> Self {
        Self {
            host: _host.to_string(),
            port: _port,
            room: Arc::new(Mutex::new(_room)),
        }
    }

    /// Return the address name.
    ///
    /// The host + port.
    fn addr(&mut self) -> String {
        self.host.to_string() + ":" + &self.port.to_string()
    }

    /// Start the server.
    ///
    /// See https://github.com/tokio-rs/tokio/blob/master/examples/chat.rs
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Listening on port {}", self.addr());

        let listener = TcpListener::bind(self.addr()).await?;

        // TODO: Add error handling.
        loop {
            let (stream, addr) = listener.accept().await?;
            let conn = Connection::new(stream, addr);
            tracing::info!("New connection from {}", addr);

            // Clone a handle to the `Shared` state for the new connection.
            let room = self.room.clone();

            tokio::spawn(async move {
                let mut channel = Channel::new(conn, &room).await;
                channel.run(&room).await;
            });
        }
    }
}
