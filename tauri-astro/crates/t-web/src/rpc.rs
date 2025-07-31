use std::str::FromStr;

use http::Uri;
use t_lib::{
    error::Result,
    log::{info, instrument},
};
use t_rpc::{
    protos::internal::{Username, internal_rpc_client::InternalRpcClient},
    tonic::Request,
    web::GrpcWebClientLayer,
};
use tower::ServiceBuilder;

use crate::grpc_web_client::Client;

#[instrument(skip_all, err)]
pub async fn get_username(username: String) -> Result<String> {
    let service = ServiceBuilder::new().layer(GrpcWebClientLayer::new()).service(Client::new());

    let uri = Uri::from_str("http://127.0.0.1:3000")?;
    let mut client = InternalRpcClient::with_origin(service, uri);

    #[cfg(feature = "encoding-gzip")]
    {
        use t_rpc::tonic::codec::CompressionEncoding::Gzip;
        client = client.accept_compressed(Gzip).send_compressed(Gzip);
    }

    let request = Request::new(Username { username });

    let response = client.get_username(request).await?;

    info!(?response);

    Ok(response.into_inner().username)
}
