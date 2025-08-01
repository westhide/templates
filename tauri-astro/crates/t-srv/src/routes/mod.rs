use axum::{Router, response::Html, routing::get};
use tower_http::cors::CorsLayer;

use crate::service::{etherscan, version};

pub async fn index() -> Html<&'static str> {
    Html("Axum Serve")
}

pub fn router<S>(state: S) -> Router
where
    S: Clone + Send + Sync + 'static,
{
    let cors = CorsLayer::permissive();

    let router = Router::new()
        .route("/", get(index))
        .route("/version", get(version::get))
        .route("/etherscan/get_block", get(etherscan::get_block_number))
        // .route("/etherscan/get_transactions", get(tx::get_transactions))
        .layer(cors);

    router.with_state(state)
}
