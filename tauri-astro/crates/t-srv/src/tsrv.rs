use std::net::SocketAddr;

use nill::{Nil, nil};
use t_lib::{error::Result, log::info};
use tokio::net::TcpListener;

use crate::routes::router;

pub async fn run() -> Result<Nil> {
    let socket: SocketAddr = "127.0.0.1:3000".parse()?;
    info!("tsrv run: {socket}");

    let listener = TcpListener::bind(socket).await?;
    let router = router();

    axum::serve(listener, router).await?;
    // .with_graceful_shutdown();

    Ok(nil)
}
