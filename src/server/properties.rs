/**
 * Copyright (c) 2024-2026 Cogru Inc.
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
use crate::constant::*;
use java_properties::PropertiesIter;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Properties file.
pub struct Properties {
    data: HashMap<String, String>,
    read: bool,
}

impl Properties {
    pub fn new(path: &str) -> Self {
        let mut prop = Self {
            data: HashMap::new(),
            read: false,
        };
        prop.read(path);
        prop
    }

    /// Read in the properties file.
    ///
    /// # Arguments
    ///
    /// * `path` - The properties file path.
    fn read(&mut self, path: &str) {
        if !Path::new(path).exists() {
            return;
        }

        let f = File::open(path).unwrap();

        let _ = PropertiesIter::new(BufReader::new(f)).read_into(|k, v| {
            self.data.insert(k, v);
        });

        self.read = true;
    }

    /// Return property value.
    ///
    /// # Arguments
    ///
    /// * `key` - The key used to find value.
    pub fn get(&self, key: &str) -> Option<String> {
        let value = self.data.get(key);
        if value.is_none() {
            return None;
        }
        Some(value.unwrap().clone())
    }

    /// Return property value or the default value when null.
    ///
    /// # Arguments
    ///
    /// * `key` - The key used to find value.
    /// * `default_value` - The fallback value.
    pub fn get_or_default(&self, key: &str, default_value: &str) -> String {
        let data = self.get(key);
        if data.is_none() {
            return default_value.to_string();
        }
        data.unwrap()
    }

    /* Customization */

    /// Return property value `cogru.BufferSize`.
    pub fn buf_size(&self) -> usize {
        let buf_size = self.get_or_default("cogru.BufferSize", &BUF_SIZE.to_string());
        buf_size.parse::<usize>().unwrap()
    }
}
