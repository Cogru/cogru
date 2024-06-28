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
use crate::constant::*;
use crate::room::*;
use crate::util::*;
use crop::Rope;

#[derive(Debug)]
pub struct File {
    path: String,       // absolute path on file system
    rel_path: String,   // relative path by room path
    chat: Chat,         // messages in this file
    view: Option<Rope>, // the file view
    use_lf: bool,       // Use LF as line endings
}

impl File {
    pub fn new(room: &Room, _path: String, _contents: Option<String>) -> Self {
        let _view = if _contents.is_none() {
            None
        } else {
            let contents = _contents.clone().unwrap();
            Some(Rope::from(contents))
        };

        let _use_lf_data = room.get_prop().get_or_default("cogru.UseLF", USE_LF);
        let _use_lf = _use_lf_data == "true";

        let mut new_file = Self {
            path: _path.clone(),
            rel_path: no_room_path(room, _path.as_str()),
            chat: Chat::new(),
            view: _view,
            use_lf: _use_lf,
        };
        // If contents is valid, meaning we trying to create the file!
        if !_contents.is_none() {
            new_file.save(); // Create the file!
        }
        new_file.load_file();
        new_file
    }

    /// Return the file path.
    pub fn path(&self) -> &String {
        &self.path
    }

    /// Return file path as relative path.
    pub fn relative_path(&self) -> String {
        self.rel_path.clone()
    }

    /// Return chat object.
    pub fn get_chat(&mut self) -> &mut Chat {
        &mut self.chat
    }

    /// Load contents into view.
    fn load_file(&mut self) {
        if !self.view.is_none() {
            return;
        }
        let contents = self.contents();
        self.view = Some(Rope::from(contents));
        self.save();
    }

    pub fn update(&mut self, add_or_delete: &String, beg: usize, end: usize, contents: &String) {
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
    pub fn buffer(&self) -> String {
        let view = self.view.clone().unwrap();
        let contents = view.to_string();
        let contents = self.normalize_le(&contents);
        contents
    }

    /// Return the file contents.
    pub fn contents(&self) -> String {
        let contents = read_to_string(&self.path);
        let contents = self.normalize_le(&contents);
        contents
    }

    /// Write the content to file.
    pub fn save(&self) {
        let contents = self.buffer();
        let _ = std::fs::write(&self.path, contents);
    }

    /// Normalize line endings by settings.
    ///
    /// # Arguments
    ///
    /// * `string` - String to normalize.
    pub fn normalize_le(&self, string: &String) -> String {
        if self.use_lf {
            return normalize_le(string);
        }
        string.to_string()
    }
}
