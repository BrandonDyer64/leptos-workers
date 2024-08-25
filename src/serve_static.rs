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

use std::{collections::HashMap, sync::Arc};

use axum::{
    http::Uri,
    http::{header, StatusCode},
    response::IntoResponse,
    Extension,
};
use once_cell::sync::Lazy;
use worker::wasm_bindgen::prelude::*;
use worker::*;

#[wasm_bindgen(module = "__STATIC_CONTENT_MANIFEST")]
extern "C" {
    #[wasm_bindgen(js_name = "default")]
    static MANIFEST: String;
}

static MANIFEST_MAP: Lazy<HashMap<&str, &str>> =
    Lazy::new(|| serde_json::from_str::<HashMap<&str, &str>>(&MANIFEST).unwrap_or_default());

#[worker::send]
pub async fn serve_static(uri: Uri, Extension(env): Extension<Arc<Env>>) -> impl IntoResponse {
    let kv_assets = env.kv("__STATIC_CONTENT").expect("KV Store");
    let asset = uri.path().trim_start_matches('/').to_string();
    let asset_str = asset.as_str();

    /* if we are using miniflare (or wrangler with --local), MANIFEST_MAP is empty and we just
    fetch the requested name of the asset from the KV store, otherwise, MANIFEST_MAP
    provides the hashed name of the asset */
    let key = MANIFEST_MAP
        .get(asset_str)
        .unwrap_or(&asset_str)
        .to_string();

    let data = kv_assets.get(&key).bytes().await;

    let mime = asset_str.rsplit_once('.').map_or_else(
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
    );

    match data {
        Ok(Some(data)) => ([(header::CONTENT_TYPE, mime)], data).into_response(),
        _ => (StatusCode::NOT_FOUND).into_response(),
    }
}
