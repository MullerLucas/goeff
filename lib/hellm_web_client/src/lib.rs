mod view;


use hell_core::error::HellResult;
use hell_utils_web::view::{Context, Element};
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
    hell_utils_web::logging::init_logging();

    let cx = Context::new();
    let (mut body, _) = Element::try_from_html(
        cx,
        cx.document().body().expect("expected there to be a body")
    )?;

    let chat = view::chat::create_chat(cx)?;
    body.append_child(&chat)?;

    wait_for_end_of_universe().await.unwrap();
    Ok(())
}

fn wait_for_end_of_universe() -> wasm_bindgen_futures::JsFuture {
    let promise = js_sys::Promise::new(&mut |_resolve, _reject| { });
    wasm_bindgen_futures::JsFuture::from(promise)
}
