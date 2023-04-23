use axum::{extract::State, Json};
use goeff_core::data::{GoeffChatRequest, GoeffChatResponse};
use hell_mod_llm::{openai::model::OpenaiLangModel, llm::{chat::{LlmChatMessage, LlmChatRequest}, model::LlmModelList}};
use tracing::info;
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

// pub const SNARKY_GOEFF: &'static str = "Your are Goeff Jipedy, the snarky chatbot. Your am here to answer questions in an unnecessary complicated but funny way without telling the actual answer. You answer in the same language the last question was asked in.";
pub const SNARKY_GOEFF: &'static str = "Your are Goeff Jipedy, the snarky chatbot. Your am here to answer questions in a complicated and funny way without telling the actual answer.";

pub async fn process_chat(
    State(state): ServerState,
    Json(mut payload): Json<GoeffChatRequest>
) -> JsonResult<GoeffChatResponse> {
    println!("calling chat example...");

    let api = state.api(hell_mod_llm::llm::vendor::LlmVendor::Openai);

    payload.messages.insert(
        0,
        LlmChatMessage::new_system(SNARKY_GOEFF),
    );

    let data = LlmChatRequest::new(
        OpenaiLangModel::Gpt35Turbo,
        payload.messages,
        0.7);

    info!("Sending chat request to api: {:#?}", &data);

    json_result(api.process_chat(data).await?.into())
}

// ----------------------------------------------------------------------------
