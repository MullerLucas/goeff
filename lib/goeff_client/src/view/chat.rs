use goeff_core::data::{GoeffChatMsg, GoeffChatRole, GoeffChatState};
use hell_core::error::HellResult;
use hell_mod_web_client::{view::{Element, ElementContainer, style::HellStyle}, console_info, console_log, console_warn};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::KeyboardEvent;
use crate::state::State;




pub async fn create_chat(state: State) -> HellResult<Element> {
    let cx = state.cx();

    let chat = Element::create_div(cx)?;
    let mut chat_elem = chat.get();
    chat_elem.add_class(HellStyle::W_0)?;

    let output_box_h = Element::create_div(cx)?;
    let mut output_box = output_box_h.get();
    output_box.add_class("chat_output_box")?;
    chat_elem.append_child(&output_box)?;

    let mut input_box = Element::create_div(cx)?.get();
    input_box.add_class("chat_input_box")?;
    chat_elem.append_child(&input_box)?;

    let input_field_h = Element::create_input(cx)?;
    let mut input_field = input_field_h.get();
    input_field.add_class("chat_input_field")?;
    input_field.set_attribute("type", "text")?;
    input_field.set_attribute("autofocus", "true")?;
    input_field.set_attribute("placeholder", "(use english, for best results ...)")?;
    input_box.append_child(&input_field)?;

    let input_send_btn = Element::create_button(cx)?;
    let mut input_send_btn_elem = input_send_btn.get();
    input_send_btn_elem.add_class("chat_input_send_btn")?;
    input_send_btn_elem.set_text_content(Some("Send"));
    input_box.append_child(&input_send_btn_elem)?;


    // get initial_chat state
    // ----------------------
    let chat_state = cx.create_signal(GoeffChatState::Initializing);

    let _ = cx.create_effect(move || {
        let chat_state = chat_state.get();
        console_log!("[CHAT]: chat state changed to '{:?}'", chat_state);

        let mut chat_elem = chat.get();
        chat_elem.remove_classes(&[
            get_class_chat_state(GoeffChatState::Initializing),
            get_class_chat_state(GoeffChatState::WaitingForUserInput),
            get_class_chat_state(GoeffChatState::WaitingForAssistantResponse),
        ]).unwrap();

        let new_class = get_class_chat_state(chat_state);
        console_warn!("[CHAT]: new state class: {:?}", new_class);
        chat_elem.add_class_uncheckd(new_class);

        let is_enabled = chat_state == GoeffChatState::WaitingForUserInput;
        let mut input_send_btn_elem = input_send_btn.get();
        input_send_btn_elem.set_disable(!is_enabled);
    });

    console_log!("[CHAT]: getting initial chat-state ...");
    let chat_data = state.api().initial_chat().await?;
    console_log!("[CHAT]: initial chat state received '{:?}'", chat_data);
    chat_state.set(GoeffChatState::WaitingForUserInput);
    let chat_data = cx.create_signal(chat_data);

    // define update procedure
    // -----------------------
    let _ = cx.create_effect(move || {
        // let c = chat_state.get();
        // console_info!("EFFECT: update_chat: {:#?}", c);
        let mut output_box = output_box_h.get();
        output_box.set_inner_html("");

        chat_data.with_mut(|d| {
            console_info!("[CHAT]: create chat from data: {:#?}", d);

            for msg in &d.messages {
                if let Some(elem) = create_chat_message_element(state, msg).unwrap() {
                    output_box.append_child_unchecked(&elem);
                }
            }
        });
    });

    input_send_btn_elem.add_event_listener("click", move |_| {
        spawn_local(async move {
            chat_state.set(GoeffChatState::WaitingForAssistantResponse);

            let mut input_field = input_field_h.get();
            console_info!("send button clicked ...");

            // append new user message
            let user_input = input_field.value();
            input_field.set_value("");

            chat_data.with_mut(|s| {
                s.messages.push(GoeffChatMsg::new_user(&user_input));
                s.messages.push(GoeffChatMsg::new_temp("..."));
            });

            // process chat
            let old_chat_state = chat_data.get();
            console_info!("Old-Chat-State: {:?}", &old_chat_state);
            let new_chat_state = state.api().process_chat(&old_chat_state).await.unwrap();
            console_info!("New-Chat-State: {:?}", new_chat_state);

            chat_data.set(new_chat_state);

            chat_state.set(GoeffChatState::WaitingForUserInput);
        });
    })?;

    input_field.add_event_listener("keydown", move |e| {
        let event: KeyboardEvent = e.dyn_into().unwrap();
        // if enter was pressed
        if event.key_code() == 13 {
            console_info!("auto-click");
            input_send_btn_elem.click();
        }
    })?;

    Ok(chat_elem)
}


pub fn create_chat_message_element(state: State, msg: &GoeffChatMsg) -> HellResult<Option<Element>> {
    if msg.role == GoeffChatRole::System {
        return Ok(None);
    }

    let cx = state.cx();
    let mut msg_elem = Element::create_div(cx)?.get();
    msg_elem.add_class("chat_msg")?;
    msg_elem.add_class(get_class_for_role(msg.role))?;

    let mut corner_elem = Element::create_div(cx)?.get();
    corner_elem.add_class("chat_msg_corner")?;
    msg_elem.append_child(&corner_elem)?;

    let mut content_elem = Element::create_span(cx)?.get();
    content_elem.set_text_content(Some(&msg.content));
    content_elem.add_class("chat_msg_content")?;
    msg_elem.append_child(&content_elem)?;

    Ok(Some(msg_elem))
}

pub fn get_class_for_role(role: GoeffChatRole) -> &'static str {
    match role {
        GoeffChatRole::Temp      => "role_temp",
        GoeffChatRole::System    => "role_system",
        GoeffChatRole::Assistant => "role_assistant",
        GoeffChatRole::User      => "role_user",
    }
}

pub fn get_class_chat_state(state: GoeffChatState) -> &'static str {
    match state {
        GoeffChatState::Initializing                => "chat_state_initializing",
        GoeffChatState::WaitingForUserInput         => "chat_state_waiting_for_user_input",
        GoeffChatState::WaitingForAssistantResponse => "chat_state_waiting_for_assistant_response",
    }
}
