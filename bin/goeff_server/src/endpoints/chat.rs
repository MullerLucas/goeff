use axum::{extract::State, Json};
use goeff_core::data::{GoeffChatData, GoeffChatMsg, GoeffChatRole};
use hell_core::error::HellErrorHelper;
use hell_mod_llm::{openai::model::OpenaiLangModel, llm::chat::LlmChatRequest};
use tracing::info;
use crate::{server::JsonResult, state::ServerState, endpoints::json_result};



pub const SNARKY_GOEFF: &str = "Your are Goeff Jipedy, the snarky chatbot. Your am here to answer questions in an unnecessary complicated but funny way without telling the actual answer.";

// ----------------------------------------------------------------------------

pub async fn initial_chat() -> JsonResult<GoeffChatData> {
    println!("[SERVER]: get initial chat ...");

    let system_msg = std::env::var("CHAT_SYSTEM_MSG").unwrap_or_else(|_| SNARKY_GOEFF.to_string());
    let system_msg = GoeffChatMsg {
        role: GoeffChatRole::System, content: system_msg
    };

    let response = GoeffChatData {
        messages: vec![system_msg],
    };

    json_result(response)
}

// ----------------------------------------------------------------------------

pub async fn extend_chat(
    State(state): ServerState,
    Json(mut data): Json<GoeffChatData>
) -> JsonResult<GoeffChatData> {
    println!("calling chat example...");

    let api = state.api(hell_mod_llm::llm::vendor::LlmVendor::Openai);

    let request = LlmChatRequest::new(
        OpenaiLangModel::Gpt35Turbo,
        GoeffChatMsg::to_llm(data.messages.clone()),
        0.7);

    info!("Sending chat request to api: {:#?}", &request);

    let response = api.process_chat(request).await?;
    let message = response.messages.into_iter()
        .map(GoeffChatMsg::from)
        .reduce(|mut acc, m| {
            acc.merge(m);
            acc
        }).ok_or_else(|| {
            eprintln!("Failed to merge chat messages!");
            HellErrorHelper::request_msg_err("Failed to merge chat messages!")
        })?;

    if let Some(msg) = data.messages.iter_mut().find(|m| m.role == GoeffChatRole::Temp) {
        *msg = message;
    }

    json_result(data)
}

// ----------------------------------------------------------------------------
