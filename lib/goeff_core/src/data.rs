use hell_mod_llm::{llm::chat::{LlmChatSuccessResponse, LlmChatMessage}};

// ----------------------------------------------------------------------------

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GoeffChatRequest {
    pub msg: String,
}

// ----------------------------------------------------------------------------

#[derive(Debug, serde:: Serialize, serde::Deserialize)]
pub struct GoeffChatResponse {
    pub mgs: LlmChatMessage,
}

impl From<LlmChatSuccessResponse> for GoeffChatResponse {
    fn from(mut val: LlmChatSuccessResponse) -> Self {
        Self {
            mgs: val.messages.remove(0),
        }
    }
}

// ----------------------------------------------------------------------------
