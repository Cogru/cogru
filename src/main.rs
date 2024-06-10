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
mod user;
mod util;
use crate::room::*;
use crate::util::*;
use clap::{arg, Arg, ArgMatches, Command};
use dunce;
use fmt::Layer;
use rpassword;
use server::properties::Properties;
use server::Server;
use std::io;
use std::io::Write;
use std::str::FromStr;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, layer::SubscriberExt};

const DOT_COGRU: &str = "./.cogru";
const PROP_FILE: &str = "./Cogru.properties";

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "8786";

/// Setup logger rotator.
///
/// https://docs.rs/tracing-appender/0.2.3/tracing_appender/non_blocking/struct.WorkerGuard.html
pub fn setup_logger(prop: &Properties) -> WorkerGuard {
    println!("Setup logger :::");
    let file_appender = tracing_appender::rolling::hourly(DOT_COGRU, "example.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let prop_log_level = prop.get_or_default("cogru.LogLevel", "DEBUG");
    let level = Level::from_str(&prop_log_level).unwrap();

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(level)
        .finish()
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
async fn start_server(prop: &Properties, port: u16, working_dir: &str, password: Option<String>) {
    println!("Start room server :::");
    let host = prop.get_or_default("cogru.Host", DEFAULT_HOST);

    let room = Room::new(working_dir, password);
    let mut server = Server::new(&host, port, room);
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

/// Setup CLI.
fn setup_cli() -> ArgMatches {
    Command::new("Cogru")
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
                .default_value(DEFAULT_PORT),
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
        .get_matches()
}

/// Program Entry
#[tokio::main]
async fn main() {
    let prop = Properties::new(&PROP_FILE);
    let prop_port = prop.get_or_default("cogru.Port", DEFAULT_PORT);

    let matches = setup_cli();

    let current_dir = get_workspace(&matches);
    let mut current_dir = to_slash(&current_dir);

    if !str::ends_with(&current_dir, "/") {
        current_dir = format!("{}/", current_dir);
    }

    let mut port = matches.get_one::<String>("port").unwrap();

    // XXX: If the port is the same as default port, we
    // assumed the user did not input the port number.
    // Let's respect the properties' port instead.
    if port == DEFAULT_PORT {
        port = &prop_port;
    }

    // Convert to u16
    let port = port.parse::<u16>().unwrap();

    let no_password = matches.get_flag("no_password");

    let password = if no_password {
        None
    } else {
        Some(get_password().expect("Confirm password doesn't match"))
    };

    // Setup logger
    let _guard = setup_logger(&prop);

    // Start the server
    start_server(&prop, port, &current_dir, password).await;
}
