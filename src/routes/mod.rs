mod mirror;
mod unrelated;

use crate::middleware::guard_coding_editor::guard_coding_editor;
use axum::{
    middleware,
    routing::{self, post},
    Router,
};
use mirror::mirror;
use unrelated::unrelated;

pub fn create_router() -> Router {
    Router::new()
        .route("/unrelated", routing::get(unrelated))
        .route(
            "/mirror",
            post(mirror).layer(middleware::from_fn(guard_coding_editor)),
        )
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
