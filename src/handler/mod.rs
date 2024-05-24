/**
 * Copyright (c) Cogru Inc. All rights reserved.
 * Licensed under the MIT License.
 * See License.txt in the project root for license information.
 */
use crate::server;

pub async fn handle(connection: &mut server::Connection, json: &str) {
    let v = serde_json::from_str(json);
    let val: serde_json::Value = v.unwrap();

    println!("{}: {}", "method", val["method"]);

    let method: &str = val["method"].as_str().unwrap();
    println!("{}: {:?}", "val", val["method"]);

    match method {
        "test" => {
            test::handle(connection, &val).await;
        }
        "ping" => {
            ping::handle(connection, &val).await;
        }
        "enter" => {
            enter::handle(connection, &val).await;
        }
        "exit" => {
            // TODO: ..
        }
        _ => {
            tracing::error!("Unkown method request: {:?}", method);
        }
    }
}

mod test {
    use crate::server;

    pub async fn handle(connection: &mut server::Connection, json: &serde_json::Value) {
        tracing::trace!("method: {:?}", json["method"]);
        connection
            .send(serde_json::json!({
                "method": "test",
                "some": "ラウトは難しいです！",
            }))
            .await;
    }
}

/// Ping pong
mod ping {
    use crate::server;
    use chrono;

    pub async fn handle(connection: &mut server::Connection, json: &serde_json::Value) {
        connection
            .send(serde_json::json!({
                "method": "pong",
                "timestamp": chrono::offset::Local::now().to_string(),
            }))
            .await;
    }
}

/// Enter session
mod enter {
    use crate::server;

    pub async fn handle(connection: &mut server::Connection, json: &serde_json::Value) {
        connection
            .send(serde_json::json!({
                "method": "enter",
            }))
            .await;
    }
}
