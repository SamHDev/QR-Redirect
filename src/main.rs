use crate::api::QRItem;
use crate::auth::{AuthList, Authorised};
use crate::config::Config;
use axum::body::Body;
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderValue, StatusCode};
use axum::response::{Redirect, Response};
use axum::routing::{get, post};
use axum::Extension;
use sha2::{Digest, Sha256};
use std::fs;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;

mod api;
mod auth;
mod config;

struct ServiceState {
    pub items: Vec<QRItem>,
    pub custom_enabled: bool,
    pub state_mut: RwLock<ServiceStateMut>,
}

#[derive(Default)]
struct ServiceStateMut {
    pub custom: Option<String>,
    pub active: Option<String>,
    pub url: Option<String>,
}

#[tokio::main]
async fn main() {
    let config_path = std::env::var("CONFIG").expect("expected env var 'CONFIG'");

    let config_bytes = fs::read_to_string(config_path).expect("failed to read config");

    let config = toml::from_str::<Config>(&config_bytes).expect("failed to parse config");

    let mut auth = AuthList::default();

    for user in config.users {
        auth.add_pair(&user.username, &user.hash)
            .expect("failed to add user");
    }

    let items = config
        .items
        .into_iter()
        .map(|x| QRItem {
            identifier: x
                .ident
                .unwrap_or_else(|| hex::encode(Sha256::digest(&x.url).as_slice())),
            label: x.label,
            url: x.url,
        })
        .collect::<Vec<QRItem>>();

    let state = ServiceState {
        items,
        custom_enabled: config.allow_custom,
        state_mut: RwLock::new(Default::default()),
    };

    let bind_addr = std::env::var("BIND").expect("expected env var 'BIND'");
    let bind_addr = SocketAddr::from_str(&bind_addr).expect("invalid env var 'BIND'");

    let router = axum::Router::new()
        .route("/configure", get(Redirect::to("/configure/")))
        .route("/configure/", get(asset_page))
        .route("/configure/style.css", get(asset_style))
        .route("/configure/script.js", get(asset_script))
        .route("/api/state", get(api::api_get_state))
        .route("/api/active", get(api::api_get_active))
        .route("/api/custom", post(api::api_write_custom))
        .route("/api/set", post(api::api_write_url))
        .route_layer(axum::middleware::from_extractor_with_state::<
            Authorised,
            Arc<AuthList>,
        >(Arc::new(auth)))
        .route("/", get(link_route))
        .layer(Extension(Arc::new(state)))
        .route("/healthcheck", get(|| async { "OK" }));

    let listener = TcpListener::bind(bind_addr)
        .await
        .expect("failed to bind socket");
    axum::serve(listener, router)
        .await
        .expect("failed to execute server")
}

async fn asset_page() -> axum::http::Response<Body> {
    let mut resp = Response::new(Body::from(include_str!("../web/configure.html")));
    resp.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
    resp
}

async fn asset_style() -> axum::http::Response<Body> {
    let mut resp = Response::new(Body::from(include_str!("../web/style.css")));
    resp.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("text/css"));
    resp
}

async fn asset_script() -> axum::http::Response<Body> {
    let mut resp = Response::new(Body::from(include_str!("../web/script.js")));
    resp.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("text/javascript"));
    resp
}

async fn link_route(
    state: Extension<Arc<ServiceState>>,
) -> Result<Redirect, (StatusCode, &'static str)> {
    state
        .state_mut
        .read()
        .expect("lock poisoned")
        .url
        .as_ref()
        .map(|url| Redirect::to(&url))
        .ok_or_else(|| (StatusCode::SERVICE_UNAVAILABLE, "QR Service Not Configured"))
}
