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
use crate::connection;
use tokio::net::TcpListener;

pub struct Server {
    host: String,
    port: u16,
    path: String,
    password: Option<String>,
    //connections: Vec<Connection>,
}

impl Server {
    pub fn new(_host: &str, _port: u16, _path: &str, _password: Option<String>) -> Self {
        Self {
            host: _host.to_string(),
            port: _port,
            path: _path.to_string(),
            password: _password,
            //connections: Vec::new(),
        }
    }

    /// Return the address name.
    ///
    /// The host + port.
    fn addr(&mut self) -> String {
        self.host.to_string() + ":" + &self.port.to_string()
    }

    /// Start the server.
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Listening on port {}", self.addr());

        let listener = TcpListener::bind(self.addr()).await?;

        loop {
            let (socket, addr) = listener.accept().await?;
            let mut conn = connection::Connection::new(socket, addr);
            tracing::info!("New connection from {}", conn.to_string());

            //self.connections.push(connection);

            tokio::spawn(async move {
                conn.run().await;
            });
        }
    }

    /// Return true when room has password
    pub fn has_password(&self) -> bool {
        self.password != None
    }
}
