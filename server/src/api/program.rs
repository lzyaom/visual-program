use crate::{db::ShareDB, models::program::Program};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
    Collection,
};
use serde_json::json;

/// 获取程序列表
pub async fn get_program_list(db: Extension<ShareDB>) -> impl IntoResponse {
    let collection: Collection<Program> = db.collection("program");

    let find_options = FindOptions::builder().sort(doc! {"create_time": -1}).build();

    let cursor = collection.find(None, find_options).await.unwrap();

    let list: Vec<Program> = cursor.try_collect().await.unwrap();

    (StatusCode::OK, Json(list)).into_response()
}

/// 通过 `id` 获取程序 [`Program`]
pub async fn get_program(db: Extension<ShareDB>, Path(id): Path<String>) -> impl IntoResponse {
    let collection: Collection<Program> = db.collection("program");

    let id = ObjectId::parse_str(&id).unwrap();

    let program = collection.find_one(doc! {"_id": id}, None).await.unwrap();

    match program {
        Some(value) => (StatusCode::OK, Json(value)).into_response(),
        None => (StatusCode::NOT_FOUND, "not found").into_response(),
    }
}

/// 获取程序 schema
pub async fn get_program_schema(db: Extension<ShareDB>, Path(id): Path<String>) -> impl IntoResponse {
    let id = ObjectId::parse_str(&id).unwrap();

    let collection: Collection<Program> = db.collection("program");

    let program = collection.find_one(doc! {"_id": id}, None).await.unwrap();

    match program {
        Some(value) => (StatusCode::OK, Json(value)).into_response(),
        None => (StatusCode::OK, Json("")).into_response(),
    }
}

/// 创建程序 schema
pub async fn create_program_schema(db: Extension<ShareDB>, Json(data): Json<Program>) -> impl IntoResponse {
    let mut data = data;

    data.id = Some(ObjectId::new());

    let collection: Collection<Program> = db.collection("program");
    let result = collection.insert_one(data, None).await;

    match result {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

/// 更新 schema
pub async fn update_program_schema(db: Extension<ShareDB>, Path(id): Path<String>, Json(data): Json<Program>) -> impl IntoResponse {
    let id = ObjectId::parse_str(&id).unwrap();

    let collection: Collection<Program> = db.collection("program");

    let result = collection.find_one_and_replace(doc! {"_id": id}, data, None).await.unwrap();

    match result {
        Some(_) => (StatusCode::OK).into_response(),
        None => (StatusCode::NOT_FOUND, Json(json!({"msg": "not found"}))).into_response(),
    }
}

/// 创建程序 [`Program`]
pub async fn create_program(db: Extension<ShareDB>, Json(data): Json<Program>) -> impl IntoResponse {
    let mut data = data;

    data.id = Some(ObjectId::new());

    let collection: Collection<Program> = db.collection("program");

    let result = collection.insert_one(data, None).await;

    match result {
        Ok(value) => {
            println!("{:#?}", value);
            (StatusCode::OK).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

/// 通过 `id` 更新程序
pub async fn update_program(db: Extension<ShareDB>, Path(id): Path<String>, Json(data): Json<Program>) -> impl IntoResponse {
    let id = ObjectId::parse_str(&id).unwrap();

    let collection: Collection<Program> = db.collection("program");

    let result = collection.find_one_and_replace(doc! {"_id": id}, data, None).await.unwrap();

    match result {
        Some(_) => (StatusCode::OK).into_response(),
        None => (StatusCode::NOT_FOUND, Json(json!({"msg": "not found"}))).into_response(),
    }
}

/// 通过 `id` 删除程序 [`Program`]
pub async fn detele_program(db: Extension<ShareDB>, Path(id): Path<String>) -> impl IntoResponse {
    let id = ObjectId::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST).unwrap();

    let collection: Collection<Program> = db.collection("program");

    let result = collection.delete_one(doc! {"_id": id}, None).await.unwrap();

    if result.deleted_count > 0 {
        (StatusCode::NO_CONTENT).into_response()
    } else {
        (StatusCode::NOT_FOUND, Json(json!({"error": "not found"}))).into_response()
    }
}

/// 运行程序 [`Program`]
pub async fn run_program() -> impl IntoResponse {
    (StatusCode::OK).into_response()
}
