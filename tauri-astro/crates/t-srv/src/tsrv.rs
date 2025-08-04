use std::{net::SocketAddr, sync::Arc};

use nill::{Nil, nil};
use t_lib::{error::Result, log::info};
use tokio::net::TcpListener;

use crate::{ctx::Context, routes::router};

pub async fn run() -> Result<Nil> {
    let ctx = Context::try_new()?;

    let socket: SocketAddr = ctx.config.url.parse()?;
    info!("tsrv run: http://{socket}");

    let listen = TcpListener::bind(socket).await?;

    let router = router(Arc::new(ctx));

    axum::serve(listen, router).await?;
    // .with_graceful_shutdown();

    Ok(nil)
}
