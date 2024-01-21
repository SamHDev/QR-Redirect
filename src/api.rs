use crate::ServiceState;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct QRItem {
    pub identifier: String,
    pub label: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct QRCustom {
    pub enabled: bool,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct QRState {
    pub active: Option<String>,
    pub custom: QRCustom,
    pub items: Vec<QRItem>,
}

pub(crate) async fn api_get_state(state: Extension<Arc<ServiceState>>) -> Json<QRState> {
    let read = state.state_mut.read().unwrap();

    Json(QRState {
        active: read.active.clone(),
        custom: QRCustom {
            enabled: state.custom_enabled,
            value: state.custom_enabled.then(|| read.custom.clone()).flatten(),
        },
        items: state.items.clone(),
    })
}

pub(crate) async fn api_get_active(state: Extension<Arc<ServiceState>>) -> Json<Option<String>> {
    Json(state.state_mut.read().unwrap().active.clone())
}

pub(crate) async fn api_write_custom(state: Extension<Arc<ServiceState>>, field: Json<String>) {
    state.state_mut.write().unwrap().custom = Some(field.0);
}

pub(crate) async fn api_write_url(state: Extension<Arc<ServiceState>>, field: Json<String>) {
    let mut lock = state.state_mut.write().unwrap();

    if field.as_str() == "@custom" {
        lock.active = Some("@custom".to_string());
        lock.url = lock.custom.clone();
    } else {
        let item = state.items.iter().find(|x| x.identifier == field.0);

        lock.active = item.map(|x| x.identifier.clone());
        lock.url = item.map(|x| x.url.clone());
    }
}
