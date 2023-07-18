use std::{fmt::Debug, ops::RangeBounds};

use crate::types::CodingEditorRequest;
use axum::{
    body::{Body, BoxBody, Full},
    extract::RawBody,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use hyper::body::{Bytes, HttpBody};
use serde::de;

pub async fn guard_coding_editor(
    request: Request<Body>,
    next: Next<Body>,
) -> Result<Response, StatusCode> {
    tracing::debug!("running middleware guard coding editor");

    // Break the request into the pieces, the parts is everything but the data (headers, etc.)
    // the body is the data itself if this was a post post request
    // This consumes the request so we'll need to re-create it later if we want to use it (we do)
    let (parts, mut body) = request.into_parts();

    tracing::debug!("we have the body: {:?}", &body);

    // We want to get the data out of the body and handle the error in the case that there isn't anything there.
    // Note that if nothing was passed in the POST request it's still possible to have a body and will "pass" this.
    let Some(body) = body.data().await else {
        tracing::debug!("body is empty?");
        return Err(StatusCode::BAD_REQUEST);
    };

    // If there was an error getting the body we want to handle that.
    let Ok(body) = body else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    // We now have body as Bytes, this dereferences to [u8]. We can convert that to a str with the str::from_utf8 method. Notice that we are using the std::str version of str.
    let string_body = std::str::from_utf8(&*body).unwrap();

    tracing::debug!("we have the body????: {string_body}");

    // we consumed the body previously, we need to re-create it so that we can pass it along to the next route handler.
    // We are using the body which is Bytes to create this. It implements From<Bytes> so this just works.
    let body = Body::from(body);

    // We can now use the body to create a new request.
    let request = Request::from_parts(parts, body);

    // We get a response by running the run method of next. This needs to be returned so that Axum is happy
    let response = next.run(request).await;
    Ok(response)
}
