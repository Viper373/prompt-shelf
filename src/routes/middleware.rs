use std::sync::Arc;

use axum::{
    body::Body,
    http::{Response, StatusCode, header::AUTHORIZATION},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use tower_http::validate_request::{ValidateRequest, ValidateRequestHeader};
use tracing::{error, info};

use super::common::AppState;

#[derive(Debug, Clone)]
pub struct JwtConf {
    pub secret: String,
    pub expire: i64,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub id: i64,
    pub email: String,
    pub iat: usize,
    pub exp: usize,
}
pub fn encode_jwt(id: i64, email: &str, jwt_conf: &JwtConf) -> anyhow::Result<String> {
    let now = Utc::now();

    let expire = Duration::hours(jwt_conf.expire);
    let claims = TokenClaims {
        id,
        email: email.to_string(),
        iat: now.timestamp() as usize,
        exp: (now + expire).timestamp() as usize,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_conf.secret.as_bytes()),
    )?;
    Ok(token)
}

pub fn decode_jwt(token: &str, jwt_conf: &JwtConf) -> anyhow::Result<TokenData<TokenClaims>> {
    let token_data = decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(jwt_conf.secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data)
}

#[derive(Clone)]
pub struct JwtAuth {
    pub conf: Arc<JwtConf>,
}

impl<B> ValidateRequest<B> for JwtAuth
where
    B: Send,
{
    type ResponseBody = Body;
    fn validate(
        &mut self,
        request: &mut axum::http::Request<B>,
    ) -> std::result::Result<(), axum::http::Response<Self::ResponseBody>> {
        let auth_err = Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Invalid or missing token"))
            .unwrap();
        let token = request
            .headers()
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .filter(|h| h.starts_with("Bearer "))
            .map(|h| &h[7..])
            .ok_or_else(|| {
                Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::from("Invalid or missing token"))
                    .unwrap()
            })?;
        let token_data = match decode_jwt(token, &self.conf) {
            Ok(data) => data,
            Err(e) => {
                error!("Failed to decode JWT: {}", e);
                return Err(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::from("Invalid or missing token"))
                    .unwrap());
            }
        };
        let now = Utc::now().timestamp() as usize;
        if token_data.claims.exp < now {
            return Err(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from("Token expired"))
                .unwrap());
        }
        request.extensions_mut().insert(token_data.claims);
        Ok(())
    }
}
