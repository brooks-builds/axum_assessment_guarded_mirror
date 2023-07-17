use axum::Json;

use crate::types::CodingEditorRequest;

pub async fn mirror(Json(body): Json<CodingEditorRequest>) -> Json<CodingEditorRequest> {
    Json(body)
}
