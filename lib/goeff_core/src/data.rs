use hell_mod_llm::llm::{chat::LlmChatMsg, role::LlmChatRole};

// ----------------------------------------------------------------------------


#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum GoeffChatRole {
    Temp,
    System,
    Assistant,
    User,
    Moderator,
}

impl From<GoeffChatRole> for LlmChatRole {
    fn from(value: GoeffChatRole) -> Self {
        match value {
            GoeffChatRole::System |
            GoeffChatRole::Moderator => Self::System,
            GoeffChatRole::Assistant => Self::Assistant,
            GoeffChatRole::User => Self::User,
            _ => unreachable!(),
        }
    }
}

impl From<LlmChatRole> for GoeffChatRole {
    fn from(value: LlmChatRole) -> Self {
        match value {
            LlmChatRole::System => Self::System,
            LlmChatRole::Assistant => Self::Assistant,
            LlmChatRole::User => Self::User,
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GoeffChatMsg {
    pub role: GoeffChatRole,
    pub content: String,
}

impl From<LlmChatMsg> for GoeffChatMsg {
    fn from(value: LlmChatMsg) -> Self {
        Self {
            role: value.role.into(),
            content: value.content
        }
    }
}

impl GoeffChatMsg {
    pub fn to_llm(value: Vec<GoeffChatMsg>) -> Vec<LlmChatMsg> {
        value.into_iter()
            .filter_map(|v| {
                if v.role == GoeffChatRole::Temp {
                    None
                } else {
                    Some(LlmChatMsg {
                        role: v.role.into(),
                        content: v.content,
                    })
                }
            })
            .collect()
    }

    pub fn new(role: GoeffChatRole, content: impl Into<String>) -> Self {
        Self { role, content: content.into(), }
    }

    pub fn new_temp(content: impl Into<String>) -> Self {
        Self::new(GoeffChatRole::Temp, content)
    }

    pub fn new_system(content: impl Into<String>) -> Self {
        Self::new(GoeffChatRole::System, content)
    }

    pub fn new_assistant(content: impl Into<String>) -> Self {
        Self::new(GoeffChatRole::Assistant, content)
    }

    pub fn new_user(content: impl Into<String>) -> Self {
        Self::new(GoeffChatRole::User, content)
    }

    pub fn new_moderator(content: impl Into<String>) -> Self {
        Self::new(GoeffChatRole::Moderator, content)
    }

    pub fn merge(&mut self, other: Self) {
        self.content.push(' ');
        self.content.push_str(&other.content);
    }
}


// ----------------------------------------------------------------------------

#[derive(Debug, Clone, serde:: Serialize, serde::Deserialize)]
pub struct GoeffChatData {
    pub messages: Vec<GoeffChatMsg>,
}

// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, serde:: Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum GoeffChatState {
    Initializing,
    WaitingForUserInput,
    WaitingForAssistantResponse,
}

// ----------------------------------------------------------------------------

#[derive(Debug, Clone, serde:: Serialize, serde::Deserialize)]
pub struct GoeffSosData {
    pub chat: GoeffChatData,
    pub round: u32,
    pub player_money: u32,
    pub assistant_money: u32,
}

// ----------------------------------------------------------------------------
