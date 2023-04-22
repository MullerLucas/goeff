use std::{sync::Arc, collections::HashMap};
use hell_mod_llm::{llm::{api::LlmApi, vendor::LlmVendor}, openai::api::OpenaiApi};


pub type ServerState = axum::extract::State<GoeffServerState>;

pub struct GoeffServerStateInner {
    api: HashMap<LlmVendor, &'static dyn LlmApi>,
}

#[derive(Clone)]
pub struct GoeffServerState {
    inner: Arc<GoeffServerStateInner>,
}

impl GoeffServerState {
    pub fn new() -> Self {
        let mut api = HashMap::new();
        api.insert(
            LlmVendor::Openai,
            Box::leak(Box::new(OpenaiApi::default())) as &'static dyn LlmApi,
        );

        Self {
            inner: Arc::new(GoeffServerStateInner {
                api,
            }),
        }
    }

    pub fn api(&self, vendor: LlmVendor) -> &'static dyn LlmApi {
        *self.inner.api.get(&vendor).expect("API not found")
    }
}



