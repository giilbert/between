use std::str::FromStr as _;

use axum::{body::Body, extract::FromRequest as _};

use super::Proxy;
use crate::{prelude::*, state::AppState};

impl Proxy {
    pub(super) async fn build_proxied_request(
        &self,
        state: &AppState,
        request: hyper::Request<Body>,
    ) -> Result<reqwest::Request> {
        let method = reqwest::Method::from_str(request.method().as_str())?;
        let new_url = hyper::Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.destination.clone())
            .path_and_query(
                request
                    .uri()
                    .path_and_query()
                    .expect("path and query for request should exist")
                    .clone(),
            )
            .build()?;

        let mut proxied_headers = reqwest::header::HeaderMap::new();
        for (key, value) in request.headers().iter() {
            if key == axum::http::header::HOST || key == axum::http::header::REFERER {
                continue;
            }
            proxied_headers.insert(
                reqwest::header::HeaderName::from_str(key.as_str())?,
                reqwest::header::HeaderValue::from_str(value.to_str()?)?,
            );
        }

        let request_body = axum::body::Bytes::from_request(request, &state).await?;

        self.client
            .request(method, new_url.to_string())
            .headers(proxied_headers)
            .body(request_body)
            .build()
            .map_err(Into::into)
    }

    pub(super) async fn build_response(
        &self,
        proxied_response: reqwest::Response,
    ) -> Result<axum::response::Response> {
        let mut response = axum::response::Response::new(Body::empty());

        *response.status_mut() = hyper::StatusCode::from_u16(proxied_response.status().as_u16())?;

        let response_headers = response.headers_mut();
        for (key, value) in proxied_response.headers().iter() {
            response_headers.insert(
                axum::http::HeaderName::from_str(key.as_str())?,
                axum::http::HeaderValue::from_str(value.to_str()?)?,
            );
        }

        *response.body_mut() = Body::from(proxied_response.bytes().await?);

        Ok(response)
    }
}
