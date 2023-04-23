use hell_mod_llm::llm::chat::{LlmChatSuccessResponse, LlmChatMessage};

// ----------------------------------------------------------------------------

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GoeffChatRequest {
    pub messages: Vec<LlmChatMessage>,
}

// ----------------------------------------------------------------------------

#[derive(Debug, Clone, serde:: Serialize, serde::Deserialize)]
pub struct GoeffChatResponse {
    pub messages: Vec<LlmChatMessage>,
}

impl From<LlmChatSuccessResponse> for GoeffChatResponse {
    fn from(val: LlmChatSuccessResponse) -> Self {
        Self {
            messages: val.messages,
        }
    }
}

// ----------------------------------------------------------------------------
