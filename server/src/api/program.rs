use crate::{db::ShareDB, models::program::Program};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions, results::UpdateResult, Collection};

/// 获取程序列表
pub async fn get_program_list(db: Extension<ShareDB>) -> impl IntoResponse {
    let collection: Collection<Program> = db.collection("programes");

    let find_options = FindOptions::builder().sort(doc! {"createTime": -1}).build();
    let cursor = collection.find(None, find_options).await.unwrap();

    let list: Vec<Program> = cursor.try_collect().await.unwrap();

    (StatusCode::OK, Json(list)).into_response()
}

/// 通过 `id` 获取程序 [`Program`]
pub async fn get_program(db: Extension<ShareDB>, Path(id): Path<String>) -> impl IntoResponse {
    let collection: Collection<Program> = db.collection("programes");

    let program = collection.find_one(doc! {"_id": id}, None).await.unwrap();

    match program {
        Some(value) => (StatusCode::OK, Json(value)).into_response(),
        None => (StatusCode::OK, Json("")).into_response(),
    }
}

/// 获取程序 schema
pub async fn get_program_schema(db: Extension<ShareDB>, Path(id): Path<String>) -> impl IntoResponse {
    let collection: Collection<Program> = db.collection("programes");

    let program = collection.find_one(doc! {"_id": id}, None).await.unwrap();

    match program {
        Some(value) => (StatusCode::OK, Json(value)).into_response(),
        None => (StatusCode::OK, Json("")).into_response(),
    }
}

/// 创建程序 schema
pub async fn create_program_schema(db: Extension<ShareDB>, Json(data): Json<Program>) -> impl IntoResponse {
    let collection: Collection<Program> = db.collection("progromes");
    let _cursor = collection.insert_one(data, None).await.unwrap();

    // if cursor.inserted_id {}
    (StatusCode::OK).into_response()
}

/// 更新 schema
pub async fn update_program_schema(db: Extension<ShareDB>) -> impl IntoResponse {
    let collection: Collection<Program> = db.collection("programes");
    let update_data = doc! {"title": ""};
    let _result: UpdateResult = collection.update_one(doc! {"_id": ""}, update_data, None).await.unwrap();

    (StatusCode::OK).into_response()
}

/// 创建程序 [`Program`]
pub async fn create_program(db: Extension<ShareDB>, Json(data): Json<Program>) -> impl IntoResponse {
    let collection: Collection<Program> = db.collection("progromes");
    let _cursor = collection.insert_one(data, None).await.unwrap();

    // if cursor.inserted_id {}
    (StatusCode::OK).into_response()
}

/// 通过 `id` 更新程序
pub async fn update_program(db: Extension<ShareDB>) -> impl IntoResponse {
    let collection: Collection<Program> = db.collection("programes");
    let update_data = doc! {"title": ""};
    let _result: UpdateResult = collection.update_one(doc! {"_id": ""}, update_data, None).await.unwrap();

    (StatusCode::OK).into_response()
}

/// 通过 `id` 删除程序 [`Program`]
pub async fn detele_program(db: Extension<ShareDB>) -> impl IntoResponse {
    let collection: Collection<Program> = db.collection("programes");
    let _result = collection.delete_one(doc! {"_id": ""}, None).await.unwrap();

    (StatusCode::OK).into_response()
}

/// 运行程序 [`Program`]
pub async fn run_program() -> impl IntoResponse {
    (StatusCode::OK).into_response()
}
