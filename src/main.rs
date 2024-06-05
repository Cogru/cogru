#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

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
mod channel;
mod chat;
mod client;
mod connection;
mod file;
mod handler;
mod room;
mod server;
use crate::room::*;
use clap::{arg, Arg, ArgMatches, Command};
use dunce;
use fmt::Layer;
use rpassword;
use server::Server;
use std::io;
use std::io::Write;
use tracing_subscriber::{fmt, layer::SubscriberExt};

const DOT_COGRU: &str = "./.cogru";

/// Setup logger rotator.
///
/// https://docs.rs/tracing-appender/0.2.3/tracing_appender/non_blocking/struct.WorkerGuard.html
pub fn setup_logger() -> tracing_appender::non_blocking::WorkerGuard {
    let file_appender = tracing_appender::rolling::hourly(DOT_COGRU, "example.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = tracing_subscriber::registry()
        .with(Layer::new().with_writer(io::stdout))
        .with(Layer::new().with_writer(non_blocking));
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global subscriber");
    guard // Don't drop this!
}

/// Start the server.
///
/// # Arguments
///
/// * `port` - port to start.
/// * `password` - password to enter the session.
async fn start_server(port: u16, working_dir: &str, password: Option<String>) {
    let _guard = setup_logger();

    let room = Room::new(working_dir, password);
    let mut server = Server::new("127.0.0.1", port, room);
    let _ = server.start().await;
}

/// Set up the session password.
fn get_password() -> Option<String> {
    print!("Set the password: ");
    io::stdout().flush().unwrap();
    let password = rpassword::read_password().unwrap();

    print!("Confirm password: ");
    io::stdout().flush().unwrap();
    let confirm = rpassword::read_password().unwrap();

    if confirm == password {
        Some(password)
    } else {
        None
    }
}

/// Return the workspace path.
///
/// This is the directory we want to watch and sync.
fn get_workspace(matches: &ArgMatches) -> String {
    let path = matches.get_one::<String>("path").unwrap();
    dunce::canonicalize(path)
        .expect("Invalid workspace")
        .display()
        .to_string()
}

/// Program Entry
#[tokio::main]
async fn main() {
    let matches = Command::new("Cogru")
        .version("0.1.0")
        .about("cogru - Where the collaboration start!?")
        .arg(
            Arg::new("path")
                .required(false)
                .help("Workspace directory")
                .default_value("."),
        )
        .arg(
            arg!(--port <VALUE>)
                .required(false)
                .help("Port number")
                .default_value("8786"),
        )
        .arg(
            Arg::new("no_password")
                .long("no-password")
                .action(clap::ArgAction::SetTrue)
                .required(false)
                .num_args(0)
                .help("Don't require password to enter the room")
                .default_value("false"),
        )
        .get_matches();

    let current_dir = get_workspace(&matches);

    let port = matches
        .get_one::<String>("port")
        .unwrap()
        .parse::<u16>()
        .unwrap();

    let no_password = matches.get_flag("no_password");

    let password = if no_password {
        None
    } else {
        Some(get_password().expect("Confirm password doesn't match"))
    };

    // Start the server
    start_server(port, &current_dir, password).await;
}
