/**
 * $File: mod.rs $
 * $Date: 2024-05-17 22:25:10 $
 * $Revision: $
 * $Creator: Jen-Chieh Shen $
 * $Notice: See LICENSE.txt for modification and distribution information
 *                   Copyright © 2024 by Shen, Jen-Chieh $
 */

pub fn handle(json: &str) {
    let v = serde_json::from_str(json);
    let val: serde_json::Value = v.unwrap();

    println!("{}: {}", "method", val["method"]);

    let method: &str = val["method"].as_str().unwrap();
    println!("{}: {:?}", "val", val["method"]);

    match method {
        "enter" => {
            enter::handle(&val);
        }
        "login" => {
            login::handle(&val);
        }
        _ => {
            tracing::error!("Unkown method request: {:?}", method);
        }
    }
}

mod enter {
    pub fn handle(json: &serde_json::Value) {
        tracing::trace!("method: {:?}", json["method"]);
    }
}

mod login {
    pub fn handle(json: &serde_json::Value) {
        tracing::trace!("method: {:?}", json["method"]);
    }
}
