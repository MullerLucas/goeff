mod view;


use hell_utils_web::{view::{Element, Runtime}, console_log};
use wasm_bindgen::prelude::*;


#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    hell_utils_web::logging::init_logging();

    let rt = Runtime::new();
    let _body: Element = rt
        .document()
        .body().expect("document should have a body")
        .try_into().unwrap();

    let test = rt.create_signal(69);
    console_log!("TEST-A: {:?}", test.get());

    rt.create_effect(move || {
        console_log!("EFFECT-IS-RUNNING: {}", test.get());
    });

    test.set(1234);
    console_log!("TEST-B: {:?}", test.get());


    wait_for_end_of_universe().await.unwrap();
    console_log!("exit...");

    Ok(())
}

#[wasm_bindgen]
pub fn version_number() -> String {
    "0.1".to_string()
}

fn wait_for_end_of_universe() -> wasm_bindgen_futures::JsFuture {
    let promise = js_sys::Promise::new(&mut |_resolve, _reject| { });
    wasm_bindgen_futures::JsFuture::from(promise)
}
