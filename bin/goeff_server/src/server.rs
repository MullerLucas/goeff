use std::{net::SocketAddr, str::FromStr};
use axum::{Router, routing::{get, post}};
use hell_core::error::HellResult;

use crate::{endpoints::{root, query_models, chat::{extend_chat, initial_chat}}, state::GoeffServerState};



pub type JsonResult<R> = HellResult<axum::Json<R>>;



pub async fn run_server() -> HellResult<()> {
    let app = Router::new()
        .route("/api/",       get(root))
        .route("/api/models", get(query_models))
        .route("/api/chat",   get(initial_chat))
        .route("/api/chat",   post(extend_chat))
        .with_state(GoeffServerState::new());

    let url = std::env::var("API_URL").unwrap();
    let addr = SocketAddr::from_str(&url).unwrap_or_else(|_| panic!("invalid address '{}'", url));
    println!("running server on addr '{:?}'", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
