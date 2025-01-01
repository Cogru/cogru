/**
 * Copyright (c) 2024-2025 Cogru Inc.
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

pub const DOT_COGRU: &str = "./.cogru";
pub const PROP_FILE: &str = "./Cogru.properties";

pub const HOST: &str = "127.0.0.1";
pub const PORT: &str = "8786";

pub const SEPARATOR_LEN: usize = "\r\n".len();
pub const BUF_SIZE: usize = 1024 * 8; // Default is 8192

pub const USE_LF: &str = "false";

/* Status */
pub const ST_SUCCESS: &str = "success";
pub const ST_FAILURE: &str = "failure";
