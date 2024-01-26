mod prelude;
mod proxy;
mod state;

use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    handler::Handler as _,
    http::uri::{Authority, Scheme},
};
use hyper::Response;
use tokio::net::TcpListener;

use prelude::*;

use crate::{
    proxy::Proxy,
    state::{AppState, AppStateInner},
};

#[tokio::main]
async fn main() -> Result<()> {
    // set up tracing with env
    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = proxy_handler.with_state(AppState(Arc::new(AppStateInner {
        proxy: Proxy::new(Scheme::HTTP, Authority::from_static("localhost:3000")),
    })));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    info!(
        "Between proxy listening on {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn proxy_handler(State(state): State<AppState>, request: Request<Body>) -> Response<Body> {
    state
        .proxy
        .handle(state.clone(), request)
        .await
        .unwrap_or_else(|err| {
            error!("Error handling request: {:?}", err);
            Response::builder()
                .status(500)
                .body(Body::from(format!("Error handling request: {:?}", err)))
                .unwrap()
        })
}
