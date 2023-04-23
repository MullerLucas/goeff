use hell_mod_web_client::view::Context;
use crate::api::GoeffApiAsync;



pub type State = GoeffClientState;

pub struct GoeffClientStateInner {
    cx: Context,
    api: GoeffApiAsync,
}

#[derive(Clone, Copy)]
pub struct GoeffClientState {
    inner: &'static GoeffClientStateInner,
}

impl GoeffClientState {
    pub fn new() -> Self {
        let cx = Context::new();
        let api = GoeffApiAsync::new(cx);

        let inner = Box::leak(Box::new(GoeffClientStateInner {
            cx,
            api,
        }));

        Self {
            inner,
        }
    }

    pub fn cx(&self) -> Context {
        self.inner.cx
    }

    pub fn api(&self) -> &GoeffApiAsync {
        &self.inner.api
    }
}
