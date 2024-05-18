/**
 * $File: packet.rs $
 * $Date: 2024-05-17 22:23:10 $
 * $Revision: $
 * $Creator: Jen-Chieh Shen $
 * $Notice: See LICENSE.txt for modification and distribution information
 *                   Copyright © 2024 by Shen, Jen-Chieh $
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
