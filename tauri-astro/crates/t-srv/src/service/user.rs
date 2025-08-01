use t_lib::{
    error::Result,
    log::{Level, instrument},
};
use t_rpc::{
    protos::internal::{Username, internal_rpc_server::InternalRpc},
    tonic::{Request, Response, Status, async_trait},
};

#[derive(Debug, Default)]
pub struct InternalRpcImpl {}

#[async_trait]
impl InternalRpc for InternalRpcImpl {
    #[instrument(level = Level::TRACE, skip_all, err)]
    async fn get_username(&self, req: Request<Username>) -> Result<Response<Username>, Status> {
        Ok(Response::new(Username { username: format!("Username: {}", req.into_inner().username) }))
    }
}
