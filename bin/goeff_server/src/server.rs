use std::net::SocketAddr;
use axum::{Router, routing::{get, post}};
use hell_core::error::HellResult;

use crate::{endpoints::{root, query_models, process_chat}, state::GoeffServerState};



pub type JsonResult<R> = HellResult<axum::Json<R>>;



pub async fn run_server() -> HellResult<()> {
    let app = Router::new()
        .route("/",       get(root))
        .route("/api/models", get(query_models))
        .route("/test", get(query_models))
        .route("/chat",   post(process_chat))
        .with_state(GoeffServerState::new());

    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // NOTE (lm): docher host addr
    let addr = SocketAddr::from(([172, 17, 0, 1], 3000));
    println!("running server on addr '{:?}'", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
