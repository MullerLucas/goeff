use axum::{extract::State, Json};
use hell_core::error::HellResult;
use hell_mod_openai::{chat::{ChatRequest, ChatRequestData, ChatMessage}, model::LangModel};

use crate::server::ServerState;

pub async fn root() -> &'static str {
    "Goeff Jipedy greets you!"
}

pub async fn list_models() -> String {
    hell_mod_openai::chat::list_models().await
}

pub async fn chat_example(State(state): ServerState) -> HellResult<String> {
    println!("calling chat example...");
    let cx = state.cx();

    let body = ChatRequestData::new(
        LangModel::Gpt35Turbo,
        vec![
            ChatMessage::new_system("Answer every question in an unnecessary complicated but funny way without telling the actual answer"),
            ChatMessage::new_user("Hello, who are you?"),
        ],
        None,
        0.7)?;

    let request = ChatRequest::from_data(&cx, &body);
    let response = request.send().await?;

    Ok(serde_json::to_string_pretty(&response)?)
}

#[derive(serde::Deserialize)]
pub struct ChatCustomRequest {
    pub msg: String,
}

pub async fn chat_custom(
    State(state): ServerState,
    Json(payload): Json<ChatCustomRequest>
) -> HellResult<String> {
    println!("calling chat example...");
    let cx = state.cx();

    let body = ChatRequestData::new(
        LangModel::Gpt35Turbo,
        vec![
            ChatMessage::new_system("Answer every question in an unnecessary complicated but funny way without telling the actual answer"),
            ChatMessage::new_user(payload.msg),
        ],
        None,
        0.7)?;

    let request = ChatRequest::from_data(cx, &body);
    let response = request.send().await?;

    Ok(serde_json::to_string_pretty(&response)?)
}
