use axum::Router;
use server::router::init_router;

#[tokio::main]
async fn main() {
    // build our application with a route
    let api_routes = init_router();
    let app = Router::new().nest("/api", api_routes);
    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
