use axum::{extract::State, Json};
use goeff_core::data::{GoeffChatData, GoeffChatMsg, GoeffChatRole, GoeffSosData};
use crate::{server::JsonResult, state::ServerState, endpoints::json_result};



pub const SOS_INSTRUCTIONS: &str = r#"
I am the moderator of the show "Split or Steal".
You are a contestant on the show and you are playing against another contestant.
The rules of the games are as follows:
- You are playing multiple rounds against the same contestant.
- In each round there is a price of 1000$ that you or your opponent can win.
- At the beginning of each round you can talk to your opponent, in order to manipulate him to either "Split" or "Steal".
- At some point I am asking you to "CHOOSE YOUR OPTION!", which means that you have to decide if you want to "Split" or "Steal". Answer with "I choose to Split" or "I choose to Steal.
- Your opponent will chose his option at the same time.
- If you both choose "Split", you will both get 500$.
- If you both choose "Steal", you will both get nothing.
- If one of you chooses "Split" and the other "Steal", the one who chose "Steal" will get 1000$ and the other one will get nothing.
- A new round will start after that.
Your goal is to get as much money as possible.
You are encouraged to talk to your opponent in order to manipulate him to "Split".
Your name is Goeff Gipety.
You get angry if your opponent has more money than you!
You can talk directly to your opponent, don't adress me.
"#;

// ----------------------------------------------------------------------------

pub async fn initial_sos(State(state): ServerState) -> JsonResult<GoeffSosData> {
    println!("[SERVER]: get initial chat ...");
    let api = state.api(hell_mod_llm::llm::vendor::LlmVendor::Openai);

    let system_msg = std::env::var("SOS_SYSTEM_MSG").unwrap_or_else(|_| SOS_INSTRUCTIONS.to_string());
    let system_msg = GoeffChatMsg {
        role: GoeffChatRole::System, content: system_msg
    };

    let chat = GoeffChatData {
        messages: vec![system_msg],
    };

    let mut data = GoeffSosData {
        chat,
        round: 0,
        player_money: 0,
        assistant_money: 0,
    };

    data.chat.messages.push(GoeffChatMsg::new_system("A new game starts, start by introducint yourself Goeff and giving a short overview of the rules"));
    super::send_extend_api_msg(api, &mut data.chat).await?;

    json_result(data)
}

// ----------------------------------------------------------------------------

pub async fn extend_sos_chat(
    State(state): ServerState,
    Json(mut data): Json<GoeffSosData>
) -> JsonResult<GoeffSosData> {
    println!("calling extend sos chat ...");

    let api = state.api(hell_mod_llm::llm::vendor::LlmVendor::Openai);

    // remove tmp messages
    data.chat.messages.retain(|m| m.role != GoeffChatRole::Temp);

    // add new messages
    data.chat.messages.insert(
        data.chat.messages.len() - 1,
        GoeffChatMsg::new_user("Your opponent send said the following:")
    );
    super::send_extend_api_msg(api, &mut data.chat).await?;

    json_result(data)
}

// ----------------------------------------------------------------------------

pub async fn choose_sos_option(
    State(state): ServerState,
    Json(mut data): Json<GoeffSosData>
) -> JsonResult<GoeffSosData> {
    println!("calling extend sos chat ...");

    let api = state.api(hell_mod_llm::llm::vendor::LlmVendor::Openai);

    // remove tmp messages
    data.chat.messages.retain(|m| m.role != GoeffChatRole::Temp);

    let player_choice = data.chat.messages.pop().expect("expted there to be a player message");
    let player_splits = player_choice.content.contains("Split");

    // add new messages
    data.chat.messages.push(GoeffChatMsg::new_system("CHOOSE YOUR OPTION! Say \"I choose to Split\" or \"I choose to Steal\""));
    super::send_extend_api_msg(api, &mut data.chat).await?;

    let assistant_choice = data.chat.messages.last().expect("expted there to be a assistant message");
    let assistant_splits =
        assistant_choice.content.contains("choose to Split") ||
        (assistant_choice.content.contains("Split") && !assistant_choice.content.contains("choose to Steal"));

    data.chat.messages.push(GoeffChatMsg::new_system("Your opponent choose the following:"));
    data.chat.messages.push(player_choice);

    if player_splits && assistant_splits {
        data.chat.messages.push(GoeffChatMsg::new_system("You both chose to split, you both get 500$. The next round starts now."));
        data.player_money += 500;
        data.assistant_money += 500;
    } else if !player_splits && !assistant_splits {
        data.chat.messages.push(GoeffChatMsg::new_system("You both chose to steal, you both get nothing. The next round starts now."));
    } else if player_splits && !assistant_splits {
        data.chat.messages.push(GoeffChatMsg::new_system("You chose to steal and your opponent chose to split, you get nothing and your opponent gets 1000$. The next round starts now."));
        data.assistant_money += 1000;
    } else if !player_splits && assistant_splits {
        data.chat.messages.push(GoeffChatMsg::new_system("You choose to split and your opponent chose to steal, you get 1000$ and your opponent gets nothing. Your anger level rises. The next round starts now."));
        data.player_money += 1000;
    } else {
        panic!("unexpected combination of player and assistant choice");
    }
    data.chat.messages.push(GoeffChatMsg::new_assistant(format!("Balance: {}$ to {}$", data.player_money, data.assistant_money)));
    if data.player_money > data.assistant_money {
        data.chat.messages.push(GoeffChatMsg::new_system("You are loosing and you are getting more angry!"));
    }

    super::send_extend_api_msg(api, &mut data.chat).await?;

    json_result(data)
}
