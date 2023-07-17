use std::{fmt::Debug, ops::RangeBounds};

use axum::{
    extract::RawBody,
    http::{Request, StatusCode},
    middleware::Next,
    Json,
};
use serde::de;

use crate::types::CodingEditorRequest;

pub async fn guard_coding_editor<'body, REQUEST_HANDLER: Debug + de::Deserialize<'body>>(
    request: Request<REQUEST_HANDLER>,
    next: Next<REQUEST_HANDLER>,
) -> Result<(), StatusCode> {
    tracing::debug!("running middleware guard coding editor");
    let body = request.body();
    tracing::debug!("body: {body:?}");
    next.run(request).await;
    Ok(())
}
