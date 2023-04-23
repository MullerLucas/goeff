use hell_mod_web_client::view::Context;

use crate::api::GoeffApi;

#[allow(unused)]
pub type State = GoeffClientState;

pub struct GoeffClientState {
    cx: Context,
    api: GoeffApi,
}

impl GoeffClientState {
    pub fn new() -> Self {
        let cx = Context::new();
        let api = GoeffApi::new(cx);

        Self {
            cx,
            api,
        }
    }

    pub fn cx(&self) -> Context {
        self.cx
    }

    pub fn api(&self) -> &GoeffApi {
        &self.api
    }
}
