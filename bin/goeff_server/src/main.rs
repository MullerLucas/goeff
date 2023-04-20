use std::net::SocketAddr;

use axum::{Router, http::StatusCode, Json, routing::{get, post}};
use serde::{Deserialize, Serialize};


#[tokio::main]
async fn main() {
    println!("starting goeff-server ...");
    dotenv::dotenv().ok();

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/joke", get(joke))
        .route("/models", get(list_models));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("stopping goeff-server ...");
}


async fn root() -> &'static str {
    "Hello, Goeff!"
}

async fn list_models() -> String {
    hell_mod_openai::list_models().await
}

async fn joke() -> String {
    hell_mod_openai::send_request().await
}
