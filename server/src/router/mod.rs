use crate::api::{
    collaboration::multi_person_collaboration,
    plugin::{create_plugin, delete_plugin, get_plugin, update_plugin},
    program::{create_program, detele_program, get_program, get_program_list, run_program, update_program},
    schema::{create_schema, get_schema, update_schema},
    user::{login, logout, register, update_user_info},
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn init_router() -> Router {
    let app = Router::new()
        .route("/program", get(get_program_list).post(create_program))
        .route(
            "/program/:id",
            get(get_program).delete(detele_program).put(update_program),
        )
        .route("/program/run", post(run_program))
        .route("/schema", post(create_schema))
        .route("/schema/:id", get(get_schema).put(update_schema))
        .route("/plugin", post(create_plugin).put(update_plugin).delete(delete_plugin))
        .route("/plugin/:id", get(get_plugin))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/user/:id", post(register).put(update_user_info))
        .route("/collaboration", post(multi_person_collaboration));
    app
}
