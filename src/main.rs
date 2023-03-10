use std::net::TcpListener;

use tracing::info;

mod setup;
use self_service::server;
use setup::config_setup;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    setup::telemetry()?;

    let settings = config_setup()?;

    info!(?settings, "config loaded");
    let listener = TcpListener::bind(("0.0.0.0", settings.port))?;

    server::start(listener).await
}
