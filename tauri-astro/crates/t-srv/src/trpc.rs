use nill::{Nil, nil};
use t_lib::{error::Result, log::info};
use t_rpc::{
    protos::internal::internal_rpc_server::InternalRpcServer, tonic::transport::Server,
    web::GrpcWebLayer,
};
use tower_http::cors::CorsLayer;

use crate::service::user::InternalRpcImpl;

pub async fn run() -> Result<Nil> {
    let socket = "127.0.0.1:3000".parse()?;
    info!("trpc run: {socket}");

    let internal_rpc = InternalRpcImpl::default();
    let mut service = InternalRpcServer::new(internal_rpc);
    #[cfg(feature = "encoding-gzip")]
    {
        use t_rpc::tonic::codec::CompressionEncoding::Gzip;
        service = service.accept_compressed(Gzip).send_compressed(Gzip);
    }

    Server::builder()
        .accept_http1(true)
        .layer(CorsLayer::permissive())
        .layer(GrpcWebLayer::new())
        .add_service(service)
        .serve(socket)
        .await?;

    Ok(nil)
}
