use axum::{Router, response::Html, routing::get};

use crate::{
    middleware::cors::cors,
    service::{alpha, etherscan, version},
};
pub async fn index() -> Html<&'static str> {
    Html("Axum Serve")
}

// TODO: Route trait
pub fn router<S>(state: S) -> Router
where
    S: Clone + Send + Sync + 'static,
{
    let router = Router::new()
        .route("/", get(index))
        .route("/version", get(version::get))
        .route("/etherscan/get_block_number", get(etherscan::get_block_number))
        .route("/etherscan/get_normal_tx", get(etherscan::get_normal_tx))
        .route("/etherscan/get_internal_tx", get(etherscan::get_internal_tx))
        .route("/etherscan/get_token_tx", get(etherscan::get_token_tx))
        .route("/alpha/get_tx", get(alpha::tx::get_tx))
        .route("/alpha/get_tx_by_date_range", get(alpha::tx::get_tx_by_date_range))
        .route("/alpha/get_total_usdt_unit", get(alpha::tx::get_total_usdt_unit))
        .route("/alpha/get_total_usdt_unit_by_date_range", get(alpha::tx::get_total_usdt_unit_by_date_range))
        // .layer(timeout(10))
        // .layer(retry(3))
        // .layer(rate(5, 1))
        .layer(cors());

    router.with_state(state)
}
