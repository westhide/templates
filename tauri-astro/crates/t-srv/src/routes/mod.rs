use axum::{Router, response::Html, routing::get};
use tower_http::cors::CorsLayer;

use crate::service::{alpha, etherscan, version};
pub async fn index() -> Html<&'static str> {
    Html("Axum Serve")
}

// TODO: Route trait
pub fn router<S>(state: S) -> Router
where
    S: Clone + Send + Sync + 'static,
{
    let cors = CorsLayer::permissive();

    let router = Router::new()
        .route("/", get(index))
        .route("/version", get(version::get))
        .route("/etherscan/get_block_number", get(etherscan::get_block_number))
        .route("/etherscan/get_normal_tx", get(etherscan::get_normal_tx))
        .route("/etherscan/get_internal_tx", get(etherscan::get_internal_tx))
        .route("/etherscan/get_token_tx", get(etherscan::get_token_tx))
        .route("/alpha/get_tx", get(alpha::tx::get))
        .layer(cors);

    router.with_state(state)
}
