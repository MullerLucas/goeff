use axum::{Json, extract::State};
use hell_mod_llm::llm::model::LlmModelList;

use crate::{server::JsonResult, state::ServerState};

pub mod chat;


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
