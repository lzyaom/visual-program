use crate::{db::ShareDB, models::program::Program};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use futures::stream::TryStreamExt;

pub async fn get_program_list(db: Extension<ShareDB>) -> impl IntoResponse {
    let collection = db.collection::<Program>("programes");

    let cursor = collection.find(None, None).await.unwrap();

    let list: Vec<Program> = cursor.try_collect().await.unwrap();

    (StatusCode::OK, Json(list)).into_response()
}
pub async fn get_program(Path(id): Path<String>) -> String {
    format!("id: {id}")
}
pub async fn get_program_schema(Path(_id): Path<String>) -> &'static str {
    "Hello, World!"
}
pub async fn create_program_schema() -> &'static str {
    "Hello, World!"
}
pub async fn update_program_schema() -> &'static str {
    "Hello, World!"
}
pub async fn create_program() -> &'static str {
    "Hello, World!"
}
pub async fn update_program() -> &'static str {
    "Hello, World!"
}
pub async fn detele_program() -> &'static str {
    "Hello, World!"
}
pub async fn run_program() -> &'static str {
    "Hello, World!"
}
