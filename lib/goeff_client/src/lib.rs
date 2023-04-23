mod view;
mod api;
mod state;

use goeff_core::data::GoeffChatRequest;
use hell_core::error::HellResult;
use hell_mod_web_client::{console_warn, utils, view::{Element, ElementContainer}};
use wasm_bindgen::prelude::*;

use crate::state::GoeffClientState;




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

    let state = GoeffClientState::new();
    let cx = state.cx();

    let (mut body, _) = Element::create_body(cx)?;

    let (mut page, _) = Element::create_div(cx)?;
    page.add_class("page")?;
    body.append_child(&page)?;

    let (mut content, _) = Element::create_div(cx)?;
    content.add_class("content")?;
    page.append_child(&content)?;

    let chat = view::chat::create_chat(cx)?;
    content.append_child(&chat)?;

    let val = state.api().query_modells().await?;
    console_warn!("TEST: {:?}", val);

    let val = state.api().process_chat(& GoeffChatRequest {
        msg: "How tall ist the Eiffel Tower?".to_string()
    }).await?;
    console_warn!("TEST: {:?}", val);

    utils::wait_for_end_of_universe().await.unwrap();
    Ok(())
}
