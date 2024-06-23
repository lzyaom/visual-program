use axum::{http::StatusCode, Extension, Json};
use serde_json::{json, Value};

use crate::{db::ShareDB, models::user::User};

pub async fn register(_db: Extension<ShareDB>, Json(_user): Json<User>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({})))
}

pub async fn login(_db: Extension<ShareDB>, Json(_user): Json<User>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({})))
}

pub async fn logout(_db: Extension<ShareDB>, Json(_user): Json<User>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({})))
}

pub async fn update_user_info(_db: Extension<ShareDB>, Json(_user): Json<User>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({})))
}
