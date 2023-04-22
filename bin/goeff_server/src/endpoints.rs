use axum::{extract::State, Json};
use hell_mod_llm::{openai::{model::OpenaiLangModel, chat::{OpenaiChatMessage, OpenaiChatSuccessResponse}}, llm::{chat::{LlmChatMessage, LlmChatRequest, LlmChatSuccessResponse}, model::LlmModelList}};
use crate::{server::JsonResult, state::ServerState};


#[inline]
fn json_result<R>(val: R) -> JsonResult<R>
where R: serde::Serialize,
{
    Ok(Json(val))
}

// ----------------------------------------------------------------------------

pub async fn root() -> &'static str {
    "Goeff Jipedy greets you!"
}

// ----------------------------------------------------------------------------

pub async fn query_models(
    State(state): ServerState,
) -> JsonResult<LlmModelList>
{
    println!("calling query models ...");
    let api = state.api(hell_mod_llm::llm::vendor::LlmVendor::Openai);
    json_result(api.querry_models().await?)
}

// ----------------------------------------------------------------------------

#[derive(Debug, serde::Deserialize)]
pub struct GoeffChatRequest {
    pub msg: String,
}

#[derive(Debug, serde:: Serialize)]
pub struct GoeffChatResponse {
    pub message: OpenaiChatMessage,
    pub total_tokens: u32,
}

impl From<OpenaiChatSuccessResponse> for GoeffChatResponse {
    fn from(mut val: OpenaiChatSuccessResponse) -> Self {
        Self {
            message: val.choices.remove(0).message,
            total_tokens: val.usage.total_tokens,
        }
    }
}

pub async fn process_chat(
    State(state): ServerState,
    Json(payload): Json<GoeffChatRequest>
) -> JsonResult<LlmChatSuccessResponse> {
    println!("calling chat example...");

    let api = state.api(hell_mod_llm::llm::vendor::LlmVendor::Openai);

    let data = LlmChatRequest::new(
        OpenaiLangModel::Gpt35Turbo,
        vec![
            LlmChatMessage::new_system("Answer every question in an unnecessary complicated but funny way without telling the actual answer"),
            LlmChatMessage::new_user(payload.msg),
        ],
        0.7);

    json_result(api.process_chat(data).await?)
}

// ----------------------------------------------------------------------------
