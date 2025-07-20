use crate::db::users::{self, Entity as Users};
use std::sync::Arc;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    routing::{delete, get, post},
};
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, IntoActiveModel,
};
use serde::{Deserialize, Serialize};
use tower_http::validate_request::ValidateRequestHeaderLayer;
use tracing::error;

use super::{
    common::{AppResponse, AppState},
    middleware::{JwtAuth, TokenClaims},
};

#[derive(Deserialize)]
pub struct ControlParams {
    enable_register: bool,
}

pub async fn is_admin(user_id: i64, conn: &DatabaseConnection) -> bool {
    let user = match Users::find_by_id(user_id).one(conn).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            error!("User not exist");
            return false;
        }
        Err(e) => {
            error!("Failed to query user: {e}");
            return false;
        }
    };
    user.role == "super_admin"
}

pub async fn allow_register(
    State(data): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
    Json(payload): Json<ControlParams>,
) -> AppResponse<String> {
    if !is_admin(claims.id, &data.sql_conn).await {
        return AppResponse::internal_err("Only super_admin can perform this action");
    }
    data.allow_register.store(
        payload.enable_register,
        std::sync::atomic::Ordering::Relaxed,
    );
    AppResponse::ok("Process finished".to_string(), None)
}

#[derive(Serialize)]
pub struct UserInfo {
    id: i64,
    username: String,
    email: String,
    role: String,
    valid: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
pub async fn all_user(
    State(data): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
) -> AppResponse<Vec<UserInfo>> {
    if !is_admin(claims.id, &data.sql_conn).await {
        return AppResponse::internal_err("Only super_admin can perform this action");
    }
    let users = match Users::find().all(&data.sql_conn).await {
        Ok(u) => u,
        Err(e) => return AppResponse::internal_err(format!("Failed to query users: {e}")),
    };
    let res = users
        .into_iter()
        .map(|u| UserInfo {
            id: u.id,
            username: u.username,
            email: u.email,
            role: u.role,
            valid: u.valid == 1,
            created_at: u.created_at,
            updated_at: u.updated_at,
        })
        .collect();
    AppResponse::ok("Query users finished".to_string(), Some(res))
}

pub async fn delete_user(
    State(data): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
    Path(user_id): Path<i64>,
) -> AppResponse<String> {
    if !is_admin(claims.id, &data.sql_conn).await {
        return AppResponse::internal_err("Only super_admin can perform this action");
    }
    match Users::delete_by_id(user_id).exec(&data.sql_conn).await {
        Ok(_) => AppResponse::ok(format!("User {user_id} has been deleted"), None),
        Err(e) => AppResponse::internal_err(format!("Failed to delete users: {e}")),
    }
}

#[derive(Deserialize)]
pub struct UserControlInfo {
    user_id: i64,
    disable: bool,
}

pub async fn user_control(
    State(data): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
    Json(payload): Json<UserControlInfo>,
) -> AppResponse<String> {
    if !is_admin(claims.id, &data.sql_conn).await {
        return AppResponse::internal_err("Only super_admin can perform this action");
    }
    let invalid_user = users::ActiveModel {
        id: Set(payload.user_id),
        valid: Set(payload.disable as i8),
        ..Default::default()
    };
    match Users::update(invalid_user).exec(&data.sql_conn).await {
        Ok(_) => AppResponse::ok("User status has been changed".to_string(), None),
        Err(e) => AppResponse::internal_err(format!("Failed to delete users: {e}")),
    }
}

#[derive(Deserialize)]
pub struct AddUserInfo {
    username: String,
    email: String,
    role: String,
    valid: bool,
    password: String,
}

pub async fn add_user(
    State(data): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
    Json(payload): Json<AddUserInfo>,
) -> AppResponse<String> {
    if !is_admin(claims.id, &data.sql_conn).await {
        return AppResponse::internal_err("Only super_admin can perform this action");
    }
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    let new_user = users::ActiveModel {
        username: Set(payload.username),
        email: Set(payload.email),
        password_hash: Set(hashed_password),
        role: Set(payload.role),
        valid: Set(payload.valid as i8),
        ..Default::default()
    };
    match Users::insert(new_user).exec(&data.sql_conn).await {
        Ok(_) => AppResponse::ok("User has been added".to_string(), None),
        Err(e) => AppResponse::internal_err(format!("Failed to add user: {e}")),
    }
}

#[derive(Deserialize)]
pub struct UpdateUserInfo {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,
    pub valid: Option<bool>,
}
pub async fn update_user(
    State(data): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
    Path(user_id): Path<i32>,
    Json(payload): Json<UpdateUserInfo>,
) -> AppResponse<String> {
    if !is_admin(claims.id, &data.sql_conn).await {
        return AppResponse::internal_err("Only super_admin can perform this action");
    }

    // 查询该用户的ActiveModel（sea-orm 通过 find_by_id）
    let mut user: users::ActiveModel = match Users::find_by_id(user_id).one(&data.sql_conn).await {
        Ok(Some(u)) => u.into_active_model(),
        Ok(None) => return AppResponse::bad_request("User not found"),
        Err(e) => return AppResponse::internal_err(format!("DB error: {e}")),
    };

    if let Some(username) = payload.username {
        user.username = Set(username);
    }
    if let Some(email) = payload.email {
        user.email = Set(email);
    }
    if let Some(role) = payload.role {
        user.role = Set(role);
    }
    if let Some(valid) = payload.valid {
        user.valid = Set(valid as i8);
    }

    // 密码单独处理，修改时哈希
    if let Some(password) = payload.password {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        user.password_hash = Set(hashed_password);
    }

    match user.update(&data.sql_conn).await {
        Ok(_) => AppResponse::ok("User updated".to_string(), None),
        Err(e) => AppResponse::internal_err(format!("Failed to update user: {e}")),
    }
}

pub fn routes(app_state: Arc<AppState>) -> Router {
    let jwt_auth = JwtAuth {
        conf: Arc::new(app_state.config.jwt_conf.clone()),
    };
    Router::new()
        .route("/register", post(allow_register))
        .route("/list/user", get(all_user))
        .route("/user/{user_id}", delete(delete_user))
        .route("/disable/user", post(user_control))
        .route("/add/user", post(add_user))
        .route("/update/user/{user_id}", post(update_user))
        .layer(ValidateRequestHeaderLayer::custom(jwt_auth))
        .with_state(app_state)
}
