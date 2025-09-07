use crate::routes::middleware::encode_jwt;
use std::sync::Arc;

use crate::db::users::{self, Entity as Users};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use axum::{Json, Router, extract::State, routing::{post, get}};
use sea_orm::{
    ActiveValue, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set, prelude::Expr,
};
use serde::{Deserialize, Serialize};

use super::common::{AppResponse, AppState};

#[derive(Deserialize)]
pub struct UserInfo {
    #[serde(default)]
    pub username: Option<String>,

    #[serde(default)]
    pub email: Option<String>,

    pub password: String,
}

#[derive(Serialize)]
pub struct ResponseUserInfo {
    id: i64,
    username: String,
    email: String,
    role: String,
    token: String,
}

#[derive(Serialize)]
pub struct RegisterFlag {
    allow_register: bool,
}

pub async fn allow_register_status(
    State(data): State<Arc<AppState>>,
) -> AppResponse<RegisterFlag> {
    let flag = data
        .allow_register
        .load(std::sync::atomic::Ordering::Relaxed);
    AppResponse::ok(
        "Query allow_register succeed".to_string(),
        Some(RegisterFlag { allow_register: flag }),
    )
}
pub async fn sign_up(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<UserInfo>,
) -> AppResponse<ResponseUserInfo> {
    if !data
        .allow_register
        .load(std::sync::atomic::Ordering::Relaxed)
    {
        return AppResponse::bad_request("Registration is disabled");
    }
    if payload.email.is_none() || payload.username.is_none() {
        return AppResponse::bad_request("Missing email or username");
    }

    let email = payload.email.unwrap();
    let username = payload.username.unwrap();

    let existing = match Users::find()
        .filter(
            Expr::col(users::Column::Email)
                .eq(email.clone())
                .or(Expr::col(users::Column::Username).eq(username.clone())),
        )
        .one(&data.sql_conn)
        .await
    {
        Ok(v) => v,
        Err(e) => return AppResponse::internal_err(format!("Failed to query db: {e}")),
    };
    if let Some(_user) = existing {
        return AppResponse::bad_request("User already exists");
    }

    // 查询用户总数
    let count = match Users::find().count(&data.sql_conn).await {
        Ok(n) => n,
        Err(e) => return AppResponse::internal_err(format!("Failed to query db: {e}")),
    };

    // 如果是第一个用户 -> super_admin，否则普通 user
    let role = if count == 0 { "super_admin" } else { "user" };
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    let user_info = users::ActiveModel {
        email: ActiveValue::Set(email.clone()),
        password_hash: ActiveValue::Set(hashed_password.clone()),
        username: ActiveValue::Set(username.clone()),
        role: ActiveValue::Set(role.to_owned()),
        ..Default::default()
    };
    match Users::insert(user_info).exec(&data.sql_conn).await {
        Ok(user) => match encode_jwt(user.last_insert_id, &email, &data.config.jwt_conf) {
            Ok(token) => {
                let response_data = ResponseUserInfo {
                    username,
                    email,
                    token,
                    id: user.last_insert_id,
                    role: role.to_string(),
                };
                AppResponse::ok("User sign up succeed.".to_string(), Some(response_data))
            }
            _ => AppResponse::internal_err("User sign up failed, encode token failed".to_string()),
        },
        Err(e) => AppResponse::internal_err(format!("User sign up failed, {e}")),
    }
}

pub async fn sign_in(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<UserInfo>,
) -> AppResponse<ResponseUserInfo> {
    if payload.email.is_none() {
        return AppResponse::bad_request("Email is required");
    }
    let email = payload.email.unwrap();
    let queried = match Users::find()
        .filter(users::Column::Email.eq(&email))
        .filter(users::Column::Valid.eq(true))
        .one(&data.sql_conn)
        .await
    {
        Ok(v) => v,
        Err(e) => return AppResponse::internal_err(format!("Failed to query db: {e}")),
    };
    if let Some(user) = queried {
        let is_valid = match PasswordHash::new(&user.password_hash) {
            Ok(parsed_hash) => Argon2::default()
                .verify_password(payload.password.as_bytes(), &parsed_hash)
                .is_ok(),
            Err(_) => false,
        };
        if is_valid {
            match encode_jwt(user.id, &user.email, &data.config.jwt_conf) {
                Ok(token) => {
                    if let Err(e) = Users::update(users::ActiveModel {
                        id: Set(user.id),
                        ..Default::default()
                    })
                    .exec(&data.sql_conn)
                    .await
                    {
                        return AppResponse::internal_err(format!(
                            "Failed to update updated_at field, {e}"
                        ));
                    }
                    let response_data = ResponseUserInfo {
                        token,
                        email,
                        username: user.username,
                        id: user.id,
                        role: user.role,
                    };
                    return AppResponse::ok(
                        "User login successfully".to_string(),
                        Some(response_data),
                    );
                }
                _ => {
                    return AppResponse::internal_err("Login failed, encode token failed");
                }
            }
        } else {
            return AppResponse::bad_request("Login failed, email or password error");
        }
    }
    AppResponse::bad_request("Login failed, user not exit")
}

pub fn routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/signin", post(sign_in))
        .route("/signup", post(sign_up))
        .route("/allow_register", get(allow_register_status))
        .with_state(app_state)
}
