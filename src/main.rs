use axum_assessment_guarded_mirror::run;

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => tracing::debug!("Server exited"),
        Err(error) => tracing::debug!("Server exited with error {error}"),
    }
}
