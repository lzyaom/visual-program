use axum::{response::IntoResponse, Extension, Json};

use crate::{db::ShareDB, models::user::User};

pub async fn register(_db: Extension<ShareDB>, Json(_user): Json<User>) -> impl IntoResponse {}

pub async fn login(_db: Extension<ShareDB>, Json(_user): Json<User>) -> impl IntoResponse {}

pub async fn logout(_db: Extension<ShareDB>, Json(_user): Json<User>) -> impl IntoResponse {}

pub async fn update_user_info(_db: Extension<ShareDB>, Json(_user): Json<User>) -> impl IntoResponse {}
