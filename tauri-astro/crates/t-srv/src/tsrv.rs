use std::{net::SocketAddr, sync::Arc};

use nill::{Nil, nil};
use t_lib::{error::Result, log::info};
use tokio::net::TcpListener;

use crate::{ctx::Context, routes::router};

pub async fn run() -> Result<Nil> {
    let socket: SocketAddr = "127.0.0.1:3000".parse()?;
    info!("tsrv run: {socket}");

    let listen = TcpListener::bind(socket).await?;

    let ctx = Context::new();
    let router = router(Arc::new(ctx));

    axum::serve(listen, router).await?;
    // .with_graceful_shutdown();

    Ok(nil)
}
