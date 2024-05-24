/**
 * $File: mod.rs $
 * $Date: 2024-05-17 22:25:10 $
 * $Revision: $
 * $Creator: Jen-Chieh Shen $
 * $Notice: See LICENSE.txt for modification and distribution information
 *                   Copyright © 2024 by Shen, Jen-Chieh $
 */
use crate::server;

pub async fn handle(connection: &mut server::Connection, json: &str) {
    let v = serde_json::from_str(json);
    let val: serde_json::Value = v.unwrap();

    println!("{}: {}", "method", val["method"]);

    let method: &str = val["method"].as_str().unwrap();
    println!("{}: {:?}", "val", val["method"]);

    match method {
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

/// Ping pong
mod ping {
    use chrono;
    use crate::server;

    pub async fn handle(connection: &mut server::Connection, json: &serde_json::Value) {
        tracing::trace!("method: {:?}", json["method"]);
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
