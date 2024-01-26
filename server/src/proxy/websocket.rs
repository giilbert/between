use axum::{
    body::Body,
    extract::{FromRequest, WebSocketUpgrade},
    http::{request::Parts, uri::Scheme},
    RequestExt,
};
use futures_util::{stream::StreamExt, SinkExt as _};
use hyper::Uri;

use super::Proxy;
use crate::{prelude::*, state::AppState};

type AxumMessage = axum::extract::ws::Message;
type TungsteniteMessage = tokio_tungstenite::tungstenite::Message;

impl Proxy {
    pub(super) async fn handle_websocket(
        &self,
        state: AppState,
        mut request: hyper::Request<Body>,
    ) -> Result<axum::response::Response<Body>> {
        let request_parts = request.extract_parts::<Parts>().await?;
        let upgrader = WebSocketUpgrade::from_request(request, &state).await?;

        let new_url = Uri::builder()
            .scheme(if self.scheme == Scheme::HTTP {
                "ws"
            } else {
                "wss"
            })
            .authority(self.destination.clone())
            .path_and_query(
                request_parts
                    .uri
                    .path_and_query()
                    .ok_or_else(|| eyre!("path and query in request should be valid"))?
                    .as_str(),
            )
            .build()?;

        let mut request = hyper::Request::builder().uri(new_url);
        request
            .headers_mut()
            .map(|headers| headers.extend(request_parts.headers.into_iter()));
        // TODO: what to do with _response?
        let (proxy_ws, _response) = tokio_tungstenite::connect_async(request.body(())?).await?;

        let (mut proxy_ws_tx, mut proxy_ws_rx) = proxy_ws.split();

        Ok(upgrader.on_upgrade(|ws| async move {
            let (mut ws_tx, mut ws_rx) = ws.split();

            loop {
                tokio::select! {
                    msg = ws_rx.next() => match msg {
                        Some(Ok(message)) => {
                            let new_message = axum_message_to_tungstenite_message(message);
                            info!("external -> proxied: {:?}", new_message);
                            proxy_ws_tx
                                .send(new_message)
                                .await
                                .expect("error forwarding external -> proxied");
                        },
                        Some(Err(e)) => {
                            error!("websocket error: {}", e);
                            break;
                        },
                        None => break,
                    },
                    msg = proxy_ws_rx.next() => match msg {
                        Some(Ok(message)) => {
                            let new_message = tungstenite_message_to_axum_message(message);
                            info!("proxied -> external: {:?}", new_message);
                            ws_tx
                                .send(new_message)
                                .await
                                .expect("error forwarding proxied -> external");
                        },
                        Some(Err(e)) => {
                            error!("websocket error: {}", e);
                            break;
                        },
                        None => break,
                    },
                }
            }
        }))
    }
}

fn axum_message_to_tungstenite_message(message: AxumMessage) -> TungsteniteMessage {
    match message {
        AxumMessage::Text(text) => TungsteniteMessage::Text(text),
        AxumMessage::Binary(bytes) => TungsteniteMessage::Binary(bytes),
        AxumMessage::Ping(bytes) => TungsteniteMessage::Ping(bytes),
        AxumMessage::Pong(bytes) => TungsteniteMessage::Pong(bytes),
        AxumMessage::Close(reason) => TungsteniteMessage::Close(reason.map(|r| {
            tokio_tungstenite::tungstenite::protocol::CloseFrame {
                code: r.code.into(),
                reason: r.reason.into(),
            }
        })),
    }
}

fn tungstenite_message_to_axum_message(message: TungsteniteMessage) -> AxumMessage {
    match message {
        TungsteniteMessage::Text(text) => AxumMessage::Text(text),
        TungsteniteMessage::Binary(bytes) => AxumMessage::Binary(bytes),
        TungsteniteMessage::Ping(bytes) => AxumMessage::Ping(bytes),
        TungsteniteMessage::Pong(bytes) => AxumMessage::Pong(bytes),
        TungsteniteMessage::Close(reason) => {
            AxumMessage::Close(reason.map(|r| axum::extract::ws::CloseFrame {
                code: r.code.into(),
                reason: r.reason.into(),
            }))
        }
        TungsteniteMessage::Frame(_) => unreachable!("Frame will never be returned by tungstenite"),
    }
}
