mod mirror;
mod unrelated;

use axum::{
    routing::{self, post},
    Router,
};
use mirror::mirror;
use unrelated::unrelated;

pub fn create_router() -> Router {
    Router::new()
        .route("/unrelated", routing::get(unrelated))
        // We are adding the guard middleware layer to the route handler itself rather than on the route chain. This is so we don't accidentally guard the wrong routes.
        .route("/mirror", post(mirror))
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
