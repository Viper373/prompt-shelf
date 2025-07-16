use crate::db::users::{self, Entity as Users};
use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    routing::{delete, get, post},
};
use chrono::{DateTime, Utc};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};
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
        return AppResponse::internal_err(format!("Only super_admin can perform this action"));
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
        return AppResponse::internal_err(format!("Only super_admin can perform this action"));
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
        return AppResponse::internal_err(format!("Only super_admin can perform this action"));
    }
    match Users::delete_by_id(user_id).exec(&data.sql_conn).await {
        Ok(_) => return AppResponse::ok(format!("User {user_id} has been deleted"), None),
        Err(e) => return AppResponse::internal_err(format!("Failed to delete users: {e}")),
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
        return AppResponse::internal_err(format!("Only super_admin can perform this action"));
    }
    let invalid_user = users::ActiveModel {
        id: Set(payload.user_id),
        valid: Set(payload.disable as i8),
        ..Default::default()
    };
    match Users::update(invalid_user).exec(&data.sql_conn).await {
        Ok(_) => return AppResponse::ok(format!("User status has been changed"), None),
        Err(e) => return AppResponse::internal_err(format!("Failed to delete users: {e}")),
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
        .layer(ValidateRequestHeaderLayer::custom(jwt_auth))
        .with_state(app_state)
}
