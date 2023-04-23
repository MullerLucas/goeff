use axum::{extract::State, Json};
use goeff_core::data::{GoeffChatRequest, GoeffChatResponse};
use hell_mod_llm::{openai::model::OpenaiLangModel, llm::{chat::{LlmChatMessage, LlmChatRequest}, model::LlmModelList}};
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

pub async fn process_chat(
    State(state): ServerState,
    Json(payload): Json<GoeffChatRequest>
) -> JsonResult<GoeffChatResponse> {
    println!("calling chat example...");

    let api = state.api(hell_mod_llm::llm::vendor::LlmVendor::Openai);

    let data = LlmChatRequest::new(
        OpenaiLangModel::Gpt35Turbo,
        vec![
            LlmChatMessage::new_system("Answer every question in an unnecessary complicated but funny way without telling the actual answer"),
            LlmChatMessage::new_user(payload.msg),
        ],
        0.7);

    json_result(api.process_chat(data).await?.into())
}

// ----------------------------------------------------------------------------
