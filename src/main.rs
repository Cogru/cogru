mod server;
mod handler;
mod packet;

use server::Server;

use std::io;
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

#[tokio::main]
async fn main() {
    let _guard = setup_logger();
    let mut server = Server::new("127.0.0.1".to_string(), 8080);
    let _ = server.start().await;
}
