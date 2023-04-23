use goeff_core::data::{GoeffChatRequest, GoeffChatResponse};
use hell_core::error::HellResult;
use hell_mod_llm::llm::model::LlmModelList;
use hell_mod_web_client::{view::Context, fetch::FetchApi};

pub struct GoeffApi {
    fetch: FetchApi,
}

impl GoeffApi {
    pub fn new(cx: Context) -> Self {
        Self {
            fetch: FetchApi::new(cx.window().clone(), "api"),
        }
    }

    #[allow(unused)]
    pub async fn query_modells(&self) -> HellResult<LlmModelList> {
        self.fetch.get("models").await
    }

    #[allow(unused)]
    pub async fn process_chat(&self, body: &GoeffChatRequest) -> HellResult<GoeffChatResponse> {
        self.fetch.post("chat", body).await
    }
}
