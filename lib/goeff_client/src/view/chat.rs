use goeff_core::data::GoeffChatRequest;
use hell_core::error::HellResult;
use hell_mod_web_client::{view::{Element, ElementContainer}, console_info};
use wasm_bindgen_futures::spawn_local;
use crate::state::State;




pub fn create_chat(state: State) -> HellResult<Element> {
    let cx = state.cx();

    let (mut chat, _) = Element::create_div(cx)?;
    chat.add_class("chat")?;

    let (mut output_box, output_box_h)= Element::create_div(cx)?;
    output_box.add_class("chat_output_box")?;
    chat.append_child(&output_box)?;

    let (mut input_box, _) = Element::create_div(cx)?;
    input_box.add_class("chat_input_box")?;
    chat.append_child(&input_box)?;

    let (mut input_field, input_field_h) = Element::create_input(cx)?;
    input_field.add_class("chat_input_field")?;
    input_field.set_attribute("type", "text")?;
    input_box.append_child(&input_field)?;

    let (mut input_send_btn, _) = Element::create_button(cx)?;
    input_send_btn.add_class("chat_input_send_btn")?;
    input_send_btn.set_text_content(Some("Send"));
    input_box.append_child(&input_send_btn)?;

    input_send_btn.add_event_listener("click", move || {
        spawn_local(async move {
            let mut output_box = output_box_h.get();
            let input_field = input_field_h.get().to_input();

            let txt = input_field.value();
            console_info!("send button clicked ...");

            let mut msg = create_chat_msg(state, &txt).expect("failed to create chat-msg");
            msg.add_class_uncheckd("chat_msg_user");
            output_box.append_child(&msg).expect("failed to append chat-msg");

            let mut assistant_msg = create_chat_msg(state, "...").expect("failed to create chat-msg");
            output_box.append_child(&assistant_msg).expect("failed to append chat-msg");

            let request = GoeffChatRequest { msg: txt, };
            let response = state.api().process_chat(&request).await.unwrap();
            console_info!("Chat-Response: {:?}", response);

            assistant_msg.set_text_content(Some(&response.mgs.content));

            assistant_msg.add_class_uncheckd("chat_msg_assistant");
        });
    })?;

    Ok(chat)
}

pub fn create_chat_msg(state: State, txt: &str) -> HellResult<Element> {
    let cx = state.cx();

    let (mut msg, _) = Element::create_div(cx)?;
    msg.add_class("chat_msg")?;
    msg.set_text_content(Some(txt));
    Ok(msg)
}
