use crate::{
    compile::{check::check_content, Compiler},
    db::ShareDB,
    models::{
        program::{Program, ProgramQuery},
        schema::Schema,
    },
};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Extension, Json,
};
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
    Collection,
};
use serde_json::{json, Value};

/// 获取程序列表
pub async fn get_program_list(
    db: Extension<ShareDB>,
    Query(params): Query<ProgramQuery>,
) -> Result<Json<Value>, StatusCode> {
    let ProgramQuery {
        title,
        status,
        page,
        size,
    } = params;

    let collection: Collection<Program> = db.collection("program");

    let status = serde_json::to_string(&status).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let filter_options = doc! { "title": title, "status": status };

    let skip_count: u64 = ((page - 1) * size).try_into().unwrap();

    let find_options = FindOptions::builder()
        .sort(doc! {"create_time": -1})
        .limit(size)
        .skip(skip_count)
        .build();

    let cursor = collection
        .find(filter_options, find_options)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let list: Vec<Program> = cursor
        .try_collect()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({"code": 0, "msg": "", "data": list})))
}

/// 通过 `id` 获取程序 [`Program`]
pub async fn get_program(db: Extension<ShareDB>, Path(id): Path<String>) -> Result<Json<Value>, StatusCode> {
    let collection: Collection<Program> = db.collection("program");

    let id = ObjectId::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let program = collection.find_one(doc! {"_id": id}, None).await.unwrap();

    match program {
        Some(value) => Ok(Json(json!({"code": 0, "msg": "", "data": value}))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// 创建程序 [`Program`]
pub async fn create_program(db: Extension<ShareDB>, Json(data): Json<Program>) -> Result<Json<Value>, StatusCode> {
    let mut data = data;

    data.id = Some(ObjectId::new());

    let collection: Collection<Program> = db.collection("program");

    let result = collection
        .insert_one(data, None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

    match result {
        Ok(_) => Ok(Json(json!({"code": 0, "msg": "create success"}))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// 通过 `id` 更新程序
pub async fn update_program(
    db: Extension<ShareDB>,
    Path(id): Path<String>,
    Json(data): Json<Program>,
) -> Result<Json<Value>, StatusCode> {
    let id = ObjectId::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let collection: Collection<Program> = db.collection("program");

    let result = collection
        .find_one_and_replace(doc! {"_id": id}, data, None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match result {
        Some(_) => Ok(Json(json!({"code": 0, "msg": "update success"}))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// 通过 `id` 删除程序 [`Program`]
pub async fn detele_program(db: Extension<ShareDB>, Path(id): Path<String>) -> Result<Json<Value>, StatusCode> {
    let id = ObjectId::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let collection: Collection<Program> = db.collection("program");

    let result = collection
        .delete_many(doc! {"_id": id}, None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.deleted_count > 0 {
        Ok(Json(json!({"code": 0, "msg": "delete success"})))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// 运行程序 [`Program`]
pub async fn run_program() -> Result<Json<Value>, Json<Value>> {
    let schema = json!({"type": "object", "title": "aaa", "properties": {}, "required": []});

    let data = json!({"title": "aa"});

    // 1. 校验内容是否符合 schema
    let result = check_content(&schema, &data);

    if let Err(error) = result {
        return Err(Json(json!({
            "code": 100,
            "msg": "fail",
            "data": error
        })));
    }

    // 2. 解析内容
    let schema_content: Schema = serde_json::from_value(schema).expect("");

    let compiler = Compiler::new();

    let code = compiler.parser.gen_to_code(&schema_content);

    // 3. 生成可执行文件
    let result = compiler.create_file(code);

    if let Err(error) = result {
        return Err(Json(json!({
            "code": 101,
            "msg": error
        })));
    }
    // 4. 运行

    Ok(Json(json!({"code": 0, "msg": "succss"})))
}
