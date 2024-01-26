mod cli;
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
use clap::Parser as _;
use hyper::Response;
use tokio::net::TcpListener;

use prelude::*;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::filter::Directive;

use crate::{
    cli::CliArgs,
    proxy::Proxy,
    state::{AppState, AppStateInner},
};

#[tokio::main]
async fn main() -> Result<()> {
    // set up tracing with env
    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .with_env_var("RUST_LOG")
                .from_env_lossy(),
        )
        .init();

    let args = CliArgs::parse();

    let destination_parts = args.destination.clone().into_parts();
    let scheme = destination_parts.scheme.unwrap_or(Scheme::HTTP);
    let authority = destination_parts
        .authority
        .ok_or_else(|| eyre!("destination must have an authority"))?;
    info!("Destination: {}://{}", scheme, authority);

    let app = proxy_handler.with_state(AppState(Arc::new(AppStateInner {
        proxy: Proxy::new(scheme, authority),
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
