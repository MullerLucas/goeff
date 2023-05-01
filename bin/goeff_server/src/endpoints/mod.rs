pub mod chat;
pub mod sos;


use goeff_core::data::{GoeffChatMsg, GoeffChatData};
use hell_core::error::{HellErrorHelper, HellResult};
use hell_mod_llm::{llm::{chat::LlmChatRequest, api::LlmApi}, openai::model::OpenaiLangModel};

use axum::{Json, extract::State};
use hell_mod_llm::llm::model::LlmModelList;

use crate::{server::JsonResult, state::ServerState};



// ----------------------------------------------------------------------------

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



pub async fn send_extend_api_msg(api: &dyn LlmApi, data: &mut GoeffChatData) -> HellResult<()> {
    let api_request = LlmChatRequest::new(
        OpenaiLangModel::Gpt35Turbo,
        GoeffChatMsg::to_llm(data.messages.clone()),
        0.7);

    println!("Sending chat-msg request to api: {:#?}", &api_request);

    let api_response = api.process_chat(api_request).await?;
    let message = api_response.messages.into_iter()
        .map(GoeffChatMsg::from)
        .reduce(|mut acc, m| {
            acc.merge(m);
            acc
        }).ok_or_else(|| {
            eprintln!("Failed to merge chat messages!");
            HellErrorHelper::request_msg_err("Failed to merge chat messages!")
        })?;

    data.messages.push(message);
    Ok(())
}

