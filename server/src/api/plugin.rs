use axum::extract::Path;

pub async fn create_plugin() -> &'static str {
    "Hello, World!"
}
pub async fn get_plugin(Path(_id): Path<String>) -> &'static str {
    "Hello, World!"
}
pub async fn update_plugin() -> &'static str {
    "Hello, World!"
}
pub async fn delete_plugin() -> &'static str {
    "Hello, World!"
}
pub async fn get_plugin_schema(Path(_id): Path<String>) -> &'static str {
    "Hello, World!"
}
pub async fn create_plugin_schema() -> &'static str {
    "Hello, World!"
}
pub async fn update_plugin_schema() -> &'static str {
    "Hello, World!"
}
