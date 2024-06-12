use crate::api::{
    plugin::{create_plugin, create_plugin_schema, delete_plugin, get_plugin, get_plugin_schema, update_plugin, update_plugin_schema},
    program::{create_program, create_program_schema, detele_program, get_program, get_program_list, get_program_schema, run_program, update_program, update_program_schema},
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn init_router() -> Router {
    let app = Router::new()
        .route("/program", get(get_program_list).post(create_program))
        .route("/program/:id", get(get_program).delete(detele_program).put(update_program))
        .route("/program/run", post(run_program))
        .route("/program/schema", post(create_program_schema))
        .route("/program/schema/:id", get(get_program_schema).put(update_program_schema))
        .route("/plugin", post(create_plugin).put(update_plugin).delete(delete_plugin))
        .route("/plugin/:id", get(get_plugin))
        .route("/plugin/schema", post(create_plugin_schema).put(update_plugin_schema))
        .route("/plugin/schema/:id", get(get_plugin_schema));
    app
}
