use axum::{extract::State, Json};
use goeff_core::data::{GoeffChatData, GoeffChatMsg, GoeffChatRole};
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

    // remove tmp messages
    data.messages.retain(|m| m.role != GoeffChatRole::Temp);

    // add new messages
    data.messages.push(GoeffChatMsg::new_user("Your opponent send sait the following:"));
    super::send_extend_api_msg(api, &mut data).await?;

    json_result(data)
}

// ----------------------------------------------------------------------------
