//! This is what serves our compiled leptos builds as well as what was included in the assets
//! folder.
//!
//! This works by loading them from Cloudflare KV by way of Worker Sites.
//! This is what the `[sites]` config was in `wrangler.toml`
//!
//! The wasm executor on the edge supplies a `__STATIC_CONTENT_MANIFEST` variable that serves as a
//! map of file names to KV keys. We then load the bytes of this data from the kv store. Finally,
//! we return this data with a mime type derived from the filename's extension (i.e. `.png` is
//! `image/png`).

use std::sync::Arc;

use axum::{
    http::Uri,
    http::{header, StatusCode},
    response::IntoResponse,
    Extension,
};
use http_body_util::BodyExt as _;
use worker::*;

pub async fn get_static_file(asset: &str, env: &Env) -> Result<Option<Vec<u8>>> {
    let asset = env
        .get_binding::<Fetcher>("ASSETS")
        .expect("ASSETS BINDING")
        .fetch(["https://example.com/", asset].concat(), None)
        .await?;
    if !asset.status().is_success() {
        return Ok(None);
    }
    let bytes = asset.into_body().collect().await?.to_bytes().to_vec();
    Ok(Some(bytes))
}

pub fn get_path_mime_type(path: &str) -> &'static str {
    path.rsplit_once('.').map_or_else(
        || "text/plain",
        |(_, ext)| match ext {
            "html" => "text/html",
            "css" => "text/css",
            "js" => "text/javascript",
            "json" => "application/json",
            "png" => "image/png",
            "jpg" => "image/jpeg",
            "jpeg" => "image/jpeg",
            "ico" => "image/x-icon",
            "wasm" => "application/wasm",
            _ => "text/plain",
        },
    )
}

#[worker::send]
pub async fn serve_static(uri: Uri, Extension(env): Extension<Arc<Env>>) -> impl IntoResponse {
    let asset = uri.path().trim_start_matches('/').to_string();
    let data = get_static_file(&asset, &env).await;
    let mime = get_path_mime_type(&asset);

    match data {
        Ok(Some(data)) => ([(header::CONTENT_TYPE, mime)], data).into_response(),
        _ => (StatusCode::NOT_FOUND).into_response(),
    }
}
