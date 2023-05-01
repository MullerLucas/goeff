mod view;
mod api;
mod state;

use hell_core::error::HellResult;
use hell_mod_web_client::{utils, view::{Element, ElementContainer, ident::HellStyle}};
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

    let _ = Element::create_html(cx)?
        .with_classes(&[
            HellStyle::height_full,
        ])?;

    let mut body = Element::create_body(cx)?
        .with_classes(&[
            HellStyle::height_full,
            HellStyle::bg_primary_400,
            HellStyle::txt_primary_400,
        ])?;

    let header = view::header::create_header(cx).await?;
    body.append_child(&header)?;

    let mut main = Element::create_main(cx)?
        .with_classes(&[
            HellStyle::height_full,
            HellStyle::mgn_auto,
            HellStyle::max_width_4xl,
            HellStyle::overflow_y_scroll,
        ])?;
    body.append_child(&main)?;

    let chat = view::chat::create_chat(state).await?;
    main.append_child(&chat)?;



    utils::wait_for_end_of_universe().await.unwrap();
    Ok(())
}
