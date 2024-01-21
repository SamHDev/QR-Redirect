use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponseParts, ResponseParts};
use base64::Engine;
use sha2::Digest;
use std::collections::BTreeMap;
use std::convert::Infallible;
use std::sync::Arc;

pub struct Authorised;

pub struct PleaseAuth;

impl IntoResponseParts for PleaseAuth {
    type Error = Infallible;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut()
            .insert("WWW-Authenticate", HeaderValue::from_static("Basic"));
        Ok(res)
    }
}

#[async_trait]
impl FromRequestParts<Arc<AuthList>> for Authorised {
    type Rejection = (StatusCode, PleaseAuth, ());

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AuthList>,
    ) -> Result<Self, Self::Rejection> {
        let field = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|x| x.starts_with("Basic").then_some(x))
            .and_then(|x| x.strip_prefix("Basic"))
            .map(str::trim)
            .and_then(|header| {
                base64::engine::general_purpose::STANDARD
                    .decode(header)
                    .ok()
            })
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, PleaseAuth, ()))?;

        let (username, password) = field
            .split_once(":")
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, PleaseAuth, ()))?;

        let hash = sha2::Sha256::digest(&password);
        let compare = state
            .members
            .get(username)
            .map(|real| hash.as_slice() == real);

        match compare {
            Some(true) => Ok(Authorised),
            _ => Err((StatusCode::FORBIDDEN, PleaseAuth, ())),
        }
    }
}

#[derive(Default)]
pub struct AuthList {
    members: BTreeMap<String, Vec<u8>>,
}

impl AuthList {
    /*pub fn add(&mut self, pair: &str) -> Result<(), ()> {
        let (username, password) = pair.split_once(':')
            .ok_or(())?;

        let password = hex::decode(password)
            .map_err(|_| ())?;

        self.members.insert(username.to_string(), password);

        Ok(())
    }*/
    pub fn add_pair(&mut self, username: &str, password: &str) -> Result<(), ()> {
        let password = hex::decode(password).map_err(|_| ())?;

        self.members.insert(username.to_string(), password);

        Ok(())
    }
}
