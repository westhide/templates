use axum::{Router, response::Html, routing::get};
use tower_http::cors::CorsLayer;

async fn handler() -> Html<&'static str> {
    Html("Axum Serve")
}

pub fn router() -> Router {
    let cors = CorsLayer::permissive();

    Router::new().layer(cors).route("/", get(handler))
}
