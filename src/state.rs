use std::{ops::Deref, sync::Arc};

use crate::proxy::Proxy;

#[derive(Debug, Clone)]
pub struct AppState(pub Arc<AppStateInner>);

impl Deref for AppState {
    type Target = Arc<AppStateInner>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct AppStateInner {
    pub proxy: Proxy,
}
