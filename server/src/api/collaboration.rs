use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

/// 多人协同编辑 [`Program`, `Plugin`]
pub async fn multi_person_collaboration() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({})))
}
