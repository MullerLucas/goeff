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

    let mut body = Element::create_body(cx)?.get();
    body.add_classes(&[
        HellStyle::bg_primary_400,
        HellStyle::text_primary_400,
    ])?;

    let header = view::header::create_header(cx).await?;
    body.append_child(&header)?;

    let mut main = Element::create_main(cx)?.get();
    main.add_classes(&[
        HellStyle::mgn_auto,
        HellStyle::mgn_top_16,
        HellStyle::max_width_4xl,
    ])?;
    main.set_text_content(Some(r#"
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj skldjflskjflksjl slfjls jlskj lsj
    "#));
    body.append_child(&main)?;

    let chat = view::chat::create_chat(state).await?;
    main.append_child(&chat)?;

    let footer = view::footer::create_footer(cx).await?;
    body.append_child(&footer)?;

    utils::wait_for_end_of_universe().await.unwrap();
    Ok(())
}
