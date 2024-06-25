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
use crate::room::*;
use crate::user::*;
use crate::util::*;
use crop::Rope;
use std::collections::HashMap;
use std::io::Write;

#[derive(Debug)]
pub struct File {
    path: String,       // absolute path
    chat: Chat,         // messages in this file
    view: Option<Rope>, // the file view
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
    pub fn path(&self) -> &String {
        &self.path
    }

    /// Return file path as relative path.
    pub fn relative_path(&self, room: &Room) -> String {
        no_room_path(room, &self.path)
    }

    /// Return chat object.
    pub fn get_chat(&mut self) -> &mut Chat {
        &mut self.chat
    }

    fn load_file(&mut self) {
        if !self.view.is_none() {
            return;
        }
        let content = read_to_string(&self.path);
        self.view = Some(Rope::from(content));
    }

    pub fn update(&mut self, add_or_delete: &String, beg: usize, end: usize, contents: &String) {
        self.load_file();
        let view = self.view.as_mut().unwrap();

        match add_or_delete.clone().as_str() {
            "add" => {
                view.insert(beg, &contents);
            }
            "delete" => {
                view.delete(beg..end);
            }
            _ => {
                unreachable!()
            }
        }
    }

    /// Return the file view.
    pub fn buffer(&mut self) -> String {
        self.load_file();
        let view = self.view.clone().unwrap();
        view.to_string()
    }

    /// Return the file contents.
    pub fn contents(&self) -> String {
        read_to_string(&self.path)
    }

    /// Write the content to file.
    pub fn save(&mut self) {
        let contents = self.buffer();
        let _ = std::fs::write(&self.path, contents);
    }
}
