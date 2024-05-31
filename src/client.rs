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
use crate::connection::*;
use crate::handler;
use crate::room::*;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub struct Client {
    connection: Connection,
    pub entered: bool,
}

impl Client {
    pub fn new(_connection: Connection) -> Self {
        Self {
            connection: _connection,
            entered: false,
        }
    }

    pub fn get_connection(&mut self) -> &mut Connection {
        &mut self.connection
    }

    pub fn get_stream(&mut self) -> &mut TcpStream {
        &mut self.connection.stream
    }
}
