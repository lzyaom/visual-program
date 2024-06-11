use axum::extract::Path;

pub async fn get_program_list() -> &'static str {
    "Hello, World!"
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
