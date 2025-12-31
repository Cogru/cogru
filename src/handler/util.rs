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
use crate::channel::*;
use crate::client::*;
use crate::server::error::*;

/// Check if the client has entered the room.
///
/// Use this when re-using client and room variables.
///
/// # Arguments
///
/// * `channel` - Send error message when not entered.
/// * `client` - The client to check to see if entered.
/// * `method` - The method id.
pub async fn check_entered(channel: &mut Channel, client: &Client, method: &str) -> bool {
    if client.entered() {
        return true;
    }

    general_error(channel, method, "You haven't entered the room yet").await;

    return false;
}

/// Return true if admin; else false and send the error message to the client.
///
/// # Arguments
///
/// * `channel` - Used when sending the error message.
/// * `client` - Client to see if it has the admin privileges.
/// * `method` - The method id.
pub async fn check_admin(channel: &mut Channel, client: &Client, method: &str) -> bool {
    if client.admin() {
        return true;
    }

    general_error(
        channel,
        method,
        "You are not the admin; only admin can operate this action",
    )
    .await;

    return false;
}
