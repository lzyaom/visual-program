use axum::{extract::Path, http::StatusCode, Extension, Json};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};
use serde_json::{json, Value};

use crate::{db::ShareDB, models::program::Program};

/// 获取程序 schema
pub async fn get_schema(db: Extension<ShareDB>, Path(id): Path<String>) -> Result<Json<Value>, StatusCode> {
    let id = ObjectId::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let collection: Collection<Program> = db.collection("schema");

    let program = collection.find_one(doc! {"_id": id}, None).await.unwrap();

    match program {
        Some(value) => Ok(Json(json!({"code": 0, "msg": "", "data": value}))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// 创建程序 schema
pub async fn create_schema(db: Extension<ShareDB>, Json(data): Json<Program>) -> Result<Json<Value>, StatusCode> {
    let mut data = data;

    data.id = Some(ObjectId::new());

    let collection: Collection<Program> = db.collection("schema");
    let result = collection
        .insert_one(data, None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

    match result {
        Ok(_) => Ok(Json(json!({"code": 0, "msg": "create success"}))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// 更新 schema
pub async fn update_schema(
    db: Extension<ShareDB>,
    Path(id): Path<String>,
    Json(data): Json<Program>,
) -> Result<Json<Value>, StatusCode> {
    let id = ObjectId::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let collection: Collection<Program> = db.collection("schema");

    let result = collection
        .find_one_and_replace(doc! {"_id": id}, data, None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match result {
        Some(_) => Ok(Json(json!({"code": 0, "msg": "update success"}))),
        None => Err(StatusCode::NOT_FOUND),
    }
}
