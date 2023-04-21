use std::{net::SocketAddr, sync::Arc};

use axum::{Router, routing::{get, post}, extract::State};
use hell_core::error::HellResult;
use hell_mod_openai::context::ApiContext;

use crate::endpoints::{root, chat_example, list_models, chat_custom};

#[derive(Default)]
pub struct GoeffServerStateInner {
    cx: ApiContext,
}

#[derive(Clone)]
pub struct GoeffServerState {
    inner: Arc<GoeffServerStateInner>,
}

pub type ServerState = State<GoeffServerState>;

impl GoeffServerState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(GoeffServerStateInner::default()),
        }
    }

    pub fn cx(&self) -> &ApiContext {
        &self.inner.cx
    }
}



pub async fn run_server() -> HellResult<()> {
    let app = Router::new()
        .route("/", get(root))
        .route("/chat/example", get(chat_example))
        .route("/chat/custom", post(chat_custom))
        .route("/models", get(list_models))
        .with_state(GoeffServerState::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
