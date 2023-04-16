use hell_core::error::HellResult;
use hell_utils_web::{view::{Element, Context, ElementContainer}, console_info};

pub fn create_chat(cx: Context) -> HellResult<Element> {
    let (mut chat, _) = Element::create_div(cx)?;

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
        let mut output_box = output_box_h.get_element();
        let input_field = input_field_h.get_input();

        let txt = input_field.value();
        let msg = create_chat_msg(cx, &txt).expect("failed to create chat-msg");
        output_box.append_child(&msg).expect("failed to append chat-msg");
        console_info!("send button clicked ...");
    })?;

    Ok(chat)
}

pub fn create_chat_msg(cx: Context, txt: &str) -> HellResult<Element> {
    let (mut msg, _) = Element::create_div(cx)?;
    msg.add_class("chat_msg")?;
    msg.set_text_content(Some(txt));
    Ok(msg)
}
