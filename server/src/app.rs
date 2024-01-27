use axum::{
    body::Body,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use hyper::{header, StatusCode, Uri};

use crate::{client_assets, state::AppState};

pub fn create_app_router() -> Router<AppState> {
    let api_router = Router::new().route("/", get(|| async { "Hello, World!" }));
    let mut router = Router::new().nest("/api", api_router);

    if client_assets::is_active() {
        router = router.fallback(serve_client_assets);
    }

    router
}

async fn serve_client_assets(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches("/");
    let mime_type = mime_guess::from_path(path).first_or_text_plain();
    let dir = client_assets::get_assets();
    let file = dir.get_file(path);

    match file {
        Some(file) => {
            let content = file.contents_utf8().unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header(
                    header::CONTENT_TYPE,
                    format!("{}; charset=utf-8", mime_type),
                )
                .body(Body::from(content))
                .unwrap()
        }
        None => {
            let content = dir
                .get_file("index.html")
                .expect("index.html should exist in client assets")
                .contents_utf8()
                .unwrap();

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                .body(Body::from(content))
                .unwrap()
        }
    }
}
