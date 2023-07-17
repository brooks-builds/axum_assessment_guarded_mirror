mod mirror;

use axum::{middleware, routing::post, Router};
use mirror::mirror;

use crate::middleware::guard_coding_editor::guard_coding_editor;

pub fn create_router() -> Router {
    Router::new()
        .route("/mirror", post(mirror))
        .layer(middleware::from_fn(guard_coding_editor))
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
