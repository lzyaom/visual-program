use axum::{extract::Path, http::StatusCode, Json};
use serde_json::{json, Value};

pub async fn create_plugin() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({})))
}

pub async fn get_plugin(Path(_id): Path<String>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({})))
}

pub async fn update_plugin() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({})))
}

pub async fn delete_plugin() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({})))
}
