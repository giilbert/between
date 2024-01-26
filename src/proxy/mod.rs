mod simple;
mod websocket;

use core::fmt::Debug;

use axum::{
    body::Body,
    http::uri::{Authority, Scheme},
};

use crate::{prelude::*, state::AppState};

#[derive(Debug)]
pub struct Proxy {
    pub scheme: Scheme,
    pub destination: Authority,
    client: reqwest::Client,
}

impl Proxy {
    pub fn new(scheme: Scheme, destination: Authority) -> Self {
        Self {
            scheme,
            destination,
            client: reqwest::Client::new(),
        }
    }

    /// Where it all begins
    pub async fn handle(
        &self,
        state: AppState,
        request: hyper::Request<Body>,
    ) -> Result<axum::response::Response<Body>> {
        if request
            .headers()
            .get(axum::http::header::UPGRADE)
            .is_some_and(|value| {
                value
                    .to_str()
                    .map(|value| value.to_lowercase() == "websocket")
                    .unwrap_or(false)
            })
        {
            return self.handle_websocket(state, request).await;
        }

        let proxied_request = self.build_proxied_request(&state, request).await?;
        let proxied_response = self.client.execute(proxied_request).await?;
        self.build_response(proxied_response).await
    }
}
