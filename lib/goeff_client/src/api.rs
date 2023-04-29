use goeff_core::data::GoeffChatData;
use hell_core::error::HellResult;
use hell_mod_llm::llm::model::LlmModelList;
use hell_mod_web_client::{view::Context, fetch::FetchAsync, console_error};



pub struct GoeffApiAsync {
    fetch: FetchAsync,
}

impl GoeffApiAsync {
    pub fn new(cx: Context) -> Self {
        Self {
            fetch: FetchAsync::new(cx.window().clone(), "api"),
        }
    }

    #[allow(unused)]
    pub async fn query_modells(&self) -> HellResult<LlmModelList> {
        self.fetch.get("models").await.map_err(|e| {
            console_error!("failed to query models: {:#?}", e);
            e
        })
    }

    #[allow(unused)]
    pub async fn initial_chat(&self) -> HellResult<GoeffChatData> {
        self.fetch.get("chat").await.map_err(|e| {
            console_error!("failed to get initial chat: {:#?}", e);
            e
        })
    }

    #[allow(unused)]
    pub async fn process_chat(&self, body: &GoeffChatData) -> HellResult<GoeffChatData> {
        self.fetch.post("chat", body).await.map_err(|e| {
            console_error!("failed to process chat: {:#?}", e);
            e
        })
    }
}
