mod app;
mod cli;
mod client_assets;
mod prelude;
mod proxy;
mod state;

use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    handler::Handler as _,
    http::uri::Scheme,
};
use clap::Parser as _;
use hyper::Response;
use tokio::net::TcpListener;

use prelude::*;
use tower_http::cors::CorsLayer;
use tracing::level_filters::LevelFilter;

use crate::{
    app::create_app_router,
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
    color_eyre::install()?;

    let args = CliArgs::parse();

    let destination_parts = args.destination.clone().into_parts();
    let scheme = destination_parts.scheme.unwrap_or(Scheme::HTTP);
    let authority = destination_parts
        .authority
        .ok_or_else(|| eyre!("destination must have an authority"))?;
    info!("Destination: {}://{}", scheme, authority);

    let state = AppState(Arc::new(AppStateInner {
        proxy: Proxy::new(scheme, authority),
    }));

    let (proxy_result, app_result) =
        tokio::join!(listen_proxy(state.clone()), listen_app(state.clone()));

    proxy_result?;
    app_result?;

    Ok(())
}

async fn listen_app(state: AppState) -> Result<()> {
    let router = create_app_router(state.clone());
    let listener = TcpListener::bind("0.0.0.0:8081").await?;
    info!("Between app listening on {}", listener.local_addr()?);
    axum::serve(
        listener,
        router.with_state(state).layer(CorsLayer::permissive()),
    )
    .await?;
    Ok(())
}

async fn listen_proxy(state: AppState) -> Result<()> {
    let proxy_service = proxy_handler.with_state(state);
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    info!("Between proxy listening on {}", listener.local_addr()?);
    axum::serve(listener, proxy_service).await?;
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
