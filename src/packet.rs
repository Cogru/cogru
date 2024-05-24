/**
 * Copyright (c) Cogru Inc. All rights reserved.
 * Licensed under the MIT License.
 * See License.txt in the project root for license information.
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
