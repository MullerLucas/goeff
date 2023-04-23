mod view;

use hell_core::error::HellResult;
use hell_mod_llm::llm::model::LlmModelList;
use hell_mod_web_client::{view::{Context, Element, ElementContainer}, console_warn, utils};
use wasm_bindgen::prelude::*;




#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    run_hell()
        .await
        .unwrap();

    Ok(())
}

#[wasm_bindgen]
pub fn version_number() -> String {
    "0.1".to_string()
}

async fn run_hell() -> HellResult<()> {
    hell_mod_web_client::logging::init_logging();

    let cx = Context::new();
    let (mut body, _) = Element::create_body(cx)?;

    let (mut page, _) = Element::create_div(cx)?;
    page.add_class("page")?;
    body.append_child(&page)?;

    let (mut content, _) = Element::create_div(cx)?;
    content.add_class("content")?;
    page.append_child(&content)?;

    let chat = view::chat::create_chat(cx)?;
    content.append_child(&chat)?;

    let val: LlmModelList = cx.fetch().get("models").await?;
    console_warn!("TEST: {:?}", val);

    // let body = GoeffChatRequest { };
    // let val: LlmChatSuccessResponse = cx.fetch().post("chat", &body).await?;
    // console_warn!("TEST: {:?}", val);

    utils::wait_for_end_of_universe().await.unwrap();
    Ok(())
}
