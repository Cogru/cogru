#![allow(dead_code)]
#![allow(unused_imports)]

mod handler;
mod packet;
mod server;
use clap::{arg, Arg, Command};
use dunce;
use rpassword;
use server::Server;
use std::io;
use std::io::Write;
use tracing_subscriber::{fmt, layer::SubscriberExt};

/// Setup logger rotator.
///
/// https://docs.rs/tracing-appender/0.2.3/tracing_appender/non_blocking/struct.WorkerGuard.html
pub fn setup_logger() -> tracing_appender::non_blocking::WorkerGuard {
    let file_appender = tracing_appender::rolling::hourly("./.log", "example.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = tracing_subscriber::registry()
        .with(fmt::Layer::new().with_writer(io::stdout))
        .with(fmt::Layer::new().with_writer(non_blocking));
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global subscriber");
    guard // Don't drop this!
}

/// Start the server.
///
/// # Arguments
///
/// * `port` - port to start.
/// * `password` - password to enter the session.
async fn start_server(port: u16, password: &str) {
    let _guard = setup_logger();

    let mut server = Server::new("127.0.0.1", port, password);
    let _ = server.start().await;
}

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

#[tokio::main]
async fn main() {
    let matches = Command::new("Cogru")
        .version("0.1.0")
        .about("Where the collaboration start!?")
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
        .get_matches();

    let path = matches.get_one::<String>("path").unwrap();
    let current_dir = dunce::canonicalize(path);

    println!("{:?}", current_dir);

    let port = matches
        .get_one::<String>("port")
        .unwrap()
        .parse::<u16>()
        .unwrap();

    let password = get_password().expect("Confirm password doesn't match");

    // Start the server
    start_server(port, &password).await;
}
