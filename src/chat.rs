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

#[derive(Debug)]
pub struct Message {
    username: String,
    content: String,
    timestamp: String,
}

impl Message {
    pub fn new(_username: &String, _content: &str) -> Self {
        Self {
            username: _username.clone(),
            content: _content.to_string(),
            timestamp: chrono::offset::Local::now().to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Chat {
    messages: Vec<Message>, // messages in this file
}

impl Chat {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    /// Add message to the chat.
    ///
    /// # Arguments
    ///
    /// * `username` - The username sent the message.
    /// * `content` - The message content.
    pub fn add_message(&mut self, username: &String, content: &String) {
        let message = Message::new(username, content);
        self.messages.push(message);
    }
}
