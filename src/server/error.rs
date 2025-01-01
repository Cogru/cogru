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
use crate::channel::*;
use crate::constant::*;

/// Send general error.
///
/// # Arguments
///
/// * `channel` - Send failure message to this channel.
/// * `method` - The method type.
/// * `key` - The missing field is the JSON key.
pub async fn general_error<S: AsRef<str>>(channel: &mut Channel, method: &str, msg: S) {
    let msg = msg.as_ref();
    channel
        .send_json(&serde_json::json!({
            "method": method,
            "message": format!("⛔ {}", msg),
            "status": ST_FAILURE,
        }))
        .await;
}

/// Send missing filed error.
///
/// # Arguments
///
/// * `channel` - Send failure message to this channel.
/// * `method` - The method type.
/// * `key` - The missing field is the JSON key.
pub async fn missing_field(channel: &mut Channel, method: &str, key: &str) {
    general_error(
        channel,
        method,
        format!("⚠ Required filed `{}` cannot be null", key),
    )
    .await;
}

/// Send obsolete notice.
///
/// # Arguments
///
/// * `channel` - Send failure message to this channel.
/// * `method` - The method type.
pub async fn obsolete_notice(channel: &mut Channel, method: &str) {
    general_error(
        channel,
        method,
        format!("📜 The method `{}` is obsoleted", method),
    )
    .await;
}
