use axum::{
  routing::{get, post},
  Router,
};
use crate::api::{
  program::{
    get_program_list,
    get_program,
    create_program,
    detele_program,
    update_program,
    run_program,
    get_program_schema,
    create_program_schema,
    update_program_schema
  },
  plugin::{
    get_plugin,
    create_plugin,
    update_plugin,
    delete_plugin,
    get_plugin_schema,
    create_plugin_schema,
    update_plugin_schema
  }
};

pub fn init_router() -> Router {
  let app = Router::new()
    .route("/program", 
    get(get_program_list)
        .post(create_program)
        .delete(detele_program)
        .put(update_program))
    .route("/program/:id", get(get_program))
    .route("/program/run", post(run_program))
    .route("/program/schema", 
      post(create_program_schema)
      .put(update_program_schema))
    .route("/program/schema/:id", get(get_program_schema))
    .route("/plugin", 
    post(create_plugin)
      .put(update_plugin)
      .delete(delete_plugin))
    .route("/plugin/:id", get(get_plugin))
    .route("/plugin/schema", 
    post(create_plugin_schema)
      .put(update_plugin_schema))
    .route("/plugin/schema/:id", get(get_plugin_schema));
  app
}