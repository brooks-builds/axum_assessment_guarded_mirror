pub mod routes;
pub mod types;

use std::net::SocketAddr;

use eyre::Result;
use routes::create_router;

pub async fn run() -> Result<()> {
    tracing_subscriber::fmt::init();
    let router = create_router();
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("Server listening on port 3000");

    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
