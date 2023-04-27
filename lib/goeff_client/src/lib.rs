mod view;
mod api;
mod state;

use hell_core::error::HellResult;
use hell_mod_web_client::{utils, view::{Element, ElementContainer}};
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

    let mut body = Element::create_body(cx)?.get();

    let mut page = Element::create_div(cx)?.get();
    page.add_class("page")?;
    body.append_child(&page)?;

    let mut content = Element::create_div(cx)?.get();
    content.add_class("content")?;
    page.append_child(&content)?;

    let mut header = Element::create_div(cx)?.get();
    header.add_class("header")?;
    content.append_child(&header)?;

    let mut title = Element::create_h1(cx)?.get();
    title.add_class("main_text")?;
    title.add_class("main_title")?;
    title.set_text_content(Some("Goeff Jipedy"));
    header.append_child(&title)?;

    let mut subtitle = Element::create_h4(cx)?.get();
    subtitle.add_class("main_text")?;
    subtitle.add_class("main_subtitle")?;
    subtitle.set_text_content(Some("Powered by Hellmut"));
    header.append_child(&subtitle)?;

    let chat = view::chat::create_chat(state)?;
    content.append_child(&chat)?;

    utils::wait_for_end_of_universe().await.unwrap();
    Ok(())
}
