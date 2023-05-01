use goeff_core::data::{GoeffChatMsg, GoeffChatRole, GoeffChatState};
use hell_core::error::HellResult;
use hell_mod_web_client::{view::{Element, ElementContainer, ident::HellStyle}, console_info, console_log};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::KeyboardEvent;
use crate::state::State;

use super::ident::GoeffStyle;




pub async fn create_chat(state: State) -> HellResult<Element> {
    let cx = state.cx();

    let (mut chat, chat_h) = Element::create_div(cx)?
        .with_classes(&[
            HellStyle::flex,
            HellStyle::flex_col,
            HellStyle::items_center,
            HellStyle::mgn_top_16,
            HellStyle::pad_4,
        ])?
        .with_handle();

    let (mut output_box, output_box_h) = Element::create_div(cx)?
        .with_classes(&[
            HellStyle::height_full,
            HellStyle::width_full,
            HellStyle::flex,
            HellStyle::flex_col,
            HellStyle::gap_2,
        ])?
        .with_handle();
    chat.append_child(&output_box)?;

    let mut controls_panel = Element::create_div(cx)?
        .with_classes(&[
            HellStyle::pos_absolute,
            HellStyle::inset_x_0,
            HellStyle::bottom_2,
        ])?;
    chat.append_child(&controls_panel)?;

    let mut inner_controls_pannel = Element::create_div(cx)?
        .with_classes(&[
            HellStyle::bg_secondary_400,
            HellStyle::max_width_3xl,
            HellStyle::mgn_auto,
            HellStyle::pad_4,
            HellStyle::rounded_md,
            HellStyle::grid,
            HellStyle::grid_cols_4,
            HellStyle::gap_2,
        ])?;
    controls_panel.append_child(&inner_controls_pannel)?;

    let (input_field, input_field_h) = Element::create_input(cx)?
        .with_attributes(&[
            ("autofocus", "true"),
            ("type", "text"),
            ("placeholder", "(use english, for best results ...)"),
        ])?
        .with_classes(&[
            HellStyle::rounded_md,
            HellStyle::col_span_3,
        ])?
        .with_handle();
    inner_controls_pannel.append_child(&input_field)?;

    let (input_send_btn, input_send_btn_h) = Element::create_button(cx)?
        .with_text_content(Some("Send"))
        .with_classes(&[
            HellStyle::bg_submit_400,
            HellStyle::txt_submit_400,
            HellStyle::pad_x_4,
            HellStyle::border_none,
            HellStyle::rounded_md,
        ])?
        .with_handle();
    inner_controls_pannel.append_child(&input_send_btn)?;


    // get initial_chat state
    // ----------------------
    let chat_state = cx.create_signal(GoeffChatState::Initializing);

    let _ = cx.create_effect(move || {
        let chat_state = chat_state.get();
        console_log!("[CHAT]: chat state changed to '{:?}'", chat_state);

        let mut chat_elem = chat_h.get();
        // chat_elem.remove_classes(&[
        //     get_class_chat_state(GoeffChatState::Initializing),
        //     get_class_chat_state(GoeffChatState::WaitingForUserInput),
        //     get_class_chat_state(GoeffChatState::WaitingForAssistantResponse),
        // ]).unwrap();
        //
        // let new_class = get_class_chat_state(chat_state);
        // console_warn!("[CHAT]: new state class: {:?}", new_class);
        // chat_elem.add_class_uncheckd(new_class);

        let is_enabled = chat_state == GoeffChatState::WaitingForUserInput;
        let mut input_send_btn_elem = input_send_btn_h.get();
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

    input_send_btn.add_event_listener("click", move |_| {
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

    // TODO (lm): improve
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


pub fn create_chat_message_element(state: State, data: &GoeffChatMsg) -> HellResult<Option<Element>> {
    if data.role == GoeffChatRole::System {
        return Ok(None);
    }

    let cx = state.cx();
    let mut msg = Element::create_div(cx)?
        .with_classes(&[
            HellStyle::pad_2,
            HellStyle::rounded_md,
        ])?;

    match data.role {
        GoeffChatRole::Temp => {
            msg.add_classes(&[
                HellStyle::bg_inactive_400,
                HellStyle::txt_inactive_400,
            ])?;
        }
        GoeffChatRole::Assistant => {
            msg.add_classes(&[
                GoeffStyle::bg_assistant_400,
                GoeffStyle::txt_assistant_400,
            ])?;
        }
        GoeffChatRole::User => {
            msg.add_classes(&[
                GoeffStyle::bg_user_400,
                GoeffStyle::txt_user_400,
            ])?;
        }
        _ => unreachable!()
    }

    let corner_elem = Element::create_div(cx)?
        .with_class("chat_msg_corner")?;
    msg.append_child(&corner_elem)?;

    let content_elem = Element::create_span(cx)?
        .with_text_content(Some(&data.content));
    msg.append_child(&content_elem)?;

    Ok(Some(msg))
}

// pub fn get_class_for_role(role: GoeffChatRole) -> &'static str {
//     match role {
//         GoeffChatRole::Temp      => "role_temp",
//         GoeffChatRole::System    => "role_system",
//         GoeffChatRole::Assistant => "role_assistant",
//         GoeffChatRole::User      => "role_user",
//     }
// }

// pub fn get_class_chat_state(state: GoeffChatState) -> &'static str {
//     match state {
//         GoeffChatState::Initializing                => "chat_state_initializing",
//         GoeffChatState::WaitingForUserInput         => "chat_state_waiting_for_user_input",
//         GoeffChatState::WaitingForAssistantResponse => "chat_state_waiting_for_assistant_response",
//     }
// }
