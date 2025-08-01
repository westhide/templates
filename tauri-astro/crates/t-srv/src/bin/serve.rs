use nill::Nil;
use t_lib::{error::Result, log::init_tracing_subscriber_log};
use t_srv::tsrv;

#[tokio::main]
pub async fn main() -> Result<Nil> {
    init_tracing_subscriber_log();

    tsrv::run().await
}
