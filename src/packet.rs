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

pub struct Packet {
    length: usize,      // the full data length
    read_length: usize, // if read length is equal length; the packet is completed
    body: String,       // the JSON dara
    buf: Vec<u8>,
}

impl Packet {
    pub fn new() -> Self {
        let packet = Self {
            length: 0,
            read_length: 0,
            body: String::new(),
            buf: Vec::new(),
        };
        packet
    }

    /// Feed the data in.
    ///
    /// # Arguments
    ///
    /// * `n` - Length of the data.
    /// * `data` - The raw data.
    pub fn feed(&mut self, mut data: Vec<u8>) {
        self.buf.append(&mut data);
    }

    pub fn is_done(&self) -> bool {
        self.length != 0 && self.length == self.read_length
    }
}
