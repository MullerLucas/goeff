use goeff_core::data::GoeffChatRequest;
use hell_core::error::HellResult;
use hell_mod_llm::llm::{chat::LlmChatMessage, role::LlmChatRole};
use hell_mod_web_client::{view::{Element, ElementContainer}, console_info};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::KeyboardEvent;
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
    input_field.set_attribute("autofocus", "true")?;
    input_field.set_attribute("placeholder", "(use english, for best results ...)")?;
    input_box.append_child(&input_field)?;

    let (mut input_send_btn, _) = Element::create_button(cx)?;
    input_send_btn.add_class("chat_input_send_btn")?;
    input_send_btn.set_text_content(Some("Send"));
    input_box.append_child(&input_send_btn)?;

    let chat_data = cx.create_signal::<Vec<LlmChatMessage>>(Vec::new());

    input_send_btn.add_event_listener("click", move |_| {
        // TODO (lm): figure out why copy does not work
        let chat_data = chat_data.clone();

        spawn_local(async move {
            let mut output_box = output_box_h.get();
            let mut input_field = input_field_h.get().to_input();
            console_info!("send button clicked ...");

            // append new user message
            let user_input = input_field.value();
            input_field.set_value("");

            let user_msg = create_user_chat_msg(state, &user_input).unwrap();
            output_box.append_child(&user_msg).unwrap();

            chat_data.with_mut(|msg| {
                msg.push(LlmChatMessage::new_user(&user_input));
            });

            // creating empty loading box, until assistant response returnded
            let mut assistant_msg_elem = create_loading_chat_msg(state).expect("failed to create chat-msg");
            output_box.append_child(&assistant_msg_elem).expect("failed to append chat-msg");

            // process chat
            let request = GoeffChatRequest { messages: chat_data.get(), };
            console_info!("Chat-Request: {:?}", &request);
            let response = state.api().process_chat(&request).await.unwrap();
            console_info!("Chat-Response: {:?}", response);

            chat_data.with_mut(|data| {
                // TODO (lm): don't clone
                for msg in response.messages.clone() {
                    data.push(msg);
                }
            });

            // update empty assistant box
            for msg in &response.messages {
                update_loading_chat_msg_with_data(state, &mut assistant_msg_elem, msg).unwrap();
            }
        });
    })?;

    input_field.add_event_listener("keydown", move |e| {
        let event: KeyboardEvent = e.dyn_into().unwrap();
        // if enter was pressed
        if event.key_code() == 13 {
            console_info!("auto-click");
            input_send_btn.click();
        }
    })?;


    Ok(chat)
}

pub fn create_user_chat_msg(state: State, user_input: &str) -> HellResult<Element> {
    let cx = state.cx();

    let (mut msg, _) = Element::create_div(cx)?;
    msg.add_class("chat_msg")?;
    msg.add_class(get_class_for_role(LlmChatRole::User))?;
    msg.set_text_content(Some(user_input));
    Ok(msg)
}

pub fn create_loading_chat_msg(state: State) -> HellResult<Element> {
    let (mut msg, _) = Element::create_div(state.cx())?;
    msg.add_class("chat_msg")?;
    msg.add_class("loading")?;
    msg.set_text_content(Some("..."));
    Ok(msg)
}

pub fn update_loading_chat_msg_with_data(_state: State, msg: &mut Element, data: &LlmChatMessage) -> HellResult<()> {
    msg.remove_class("loading")?;
    msg.add_class(get_class_for_role(data.role))?;
    msg.set_text_content(Some(&data.content));

    Ok(())
}

pub fn get_class_for_role(role: LlmChatRole) -> &'static str {
    match role {
        LlmChatRole::System => "role_system",
        LlmChatRole::Assistant => "role_assistant",
        LlmChatRole::User => "role_user",
    }
}
