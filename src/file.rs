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
use jumprope::{JumpRope, JumpRopeBuf};
use std::collections::HashMap;
use std::io::Write;

#[derive(Debug)]
pub struct File {
    path: String,              // absolute path
    chat: Chat,                // messages in this file
    view: Option<JumpRopeBuf>, // the file view
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
        let content = read_to_string(&self.path);
        self.view = Some(JumpRopeBuf::from(content));
    }

    pub fn update(&mut self, add_or_delete: &String, beg: usize, end: usize, contents: &String) {
        self.load_file(); // ensure read
        let view = self.view.as_mut().unwrap();

        match add_or_delete.clone().as_str() {
            "add" => {
                view.insert(beg, &contents);
            }
            "delete" => {
                view.remove(beg..end);
            }
            _ => {
                unreachable!()
            }
        }
    }

    /// Return the file contents.
    pub fn contents(&mut self) -> String {
        self.load_file(); // ensure read
        let view = self.view.clone().unwrap();
        view.to_string()
    }

    /// Write the content to file.
    pub fn save(&mut self) {
        let contents = self.contents();
        let _ = std::fs::write(&self.path, contents);
    }
}
