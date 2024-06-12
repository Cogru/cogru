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
use crate::chat::*;
use crate::user::*;
use crate::util::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct File {
    path: String,         // absolute path
    chat: Chat,           // messages in this file
    view: Option<String>, // the file view
}

impl File {
    pub fn new(_path: String) -> Self {
        Self {
            path: _path,
            chat: Chat::new(),
            view: None,
        }
    }

    /// Return the file path.
    pub fn get_path(&self) -> &String {
        &self.path
    }

    /// Return chat object.
    pub fn get_chat(&mut self) -> &mut Chat {
        &mut self.chat
    }

    fn load_file(&mut self) {
        if !self.view.is_none() {
            return;
        }
        self.view = Some(read_to_string(&self.path));
    }

    pub fn update(&mut self, add_or_delete: &String, beg: u64, end: u64, content: &String) {
        self.load_file(); // ensure read

        match add_or_delete.clone().as_str() {
            "add" => {
                // TODO: ..
                //self.view.insert(content, beg);
            }
            "delete" => {
                // TODO: ..
            }
            _ => {
                unreachable!()
            }
        }
    }

    /// Write the content to file.
    pub fn save(&self) {
        let _ = std::fs::write(&self.path, &self.view.clone().unwrap());
    }
}
