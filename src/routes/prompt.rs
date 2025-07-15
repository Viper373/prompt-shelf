use std::sync::Arc;

use crate::db::prompts::{self, Entity as PromptData};
use anyhow::{Result, anyhow};
use axum::{Extension, Json, Router, extract::State, routing::post};
use sea_orm::{ActiveValue::Set, EntityTrait};
use sea_orm::{ColumnTrait, DatabaseConnection, QueryFilter};
use serde::{Deserialize, Serialize};
use tower_http::validate_request::ValidateRequestHeaderLayer;
use tracing::info;

use super::common::PromptCommit;
use super::finder::find_config;
use super::{
    common::{AppResponse, AppState, Prompts},
    middleware::{JwtAuth, TokenClaims},
};

#[derive(Debug, Deserialize)]
pub struct PromptInfo {
    name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateResponse {
    id: u64,
}

pub async fn create_prompt(
    State(data): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
    Json(payload): Json<PromptInfo>,
) -> AppResponse<CreateResponse> {
    let prompt = Prompts::new(payload.name);
    match prompt.save().await {
        Ok(()) => info!("Prompt {} saved.", prompt.id()),
        Err(e) => {
            return AppResponse::internal_err(format!(
                "Failed to save Prompt {}, {}",
                prompt.id(),
                e
            ));
        }
    };
    let prompt_model = prompts::ActiveModel {
        file_key: Set(prompt.id()),
        user_id: Set(Some(claims.id)),
        ..Default::default()
    };
    match PromptData::insert(prompt_model).exec(&data.sql_conn).await {
        Ok(pt) => AppResponse::ok(
            "Create Prompt finished.".to_string(),
            Some(CreateResponse {
                id: pt.last_insert_id,
            }),
        ),
        Err(e) => AppResponse::internal_err(format!("Failed to add prompt: {}", e)),
    }
}

pub async fn find_prompt(
    conn: &DatabaseConnection,
    user_id: i64,
    prompt_id: u64,
) -> Result<Prompts> {
    let prompt = match PromptData::find_by_id(prompt_id)
        .filter(prompts::Column::UserId.eq(user_id))
        .one(conn)
        .await
    {
        Ok(Some(p)) => p,
        Ok(None) => return Err(anyhow!("Prompt id not exist!")),
        Err(e) => return Err(anyhow!("Failed to query db: {}", e)),
    };
    let prompt_config_path = find_config(&prompt.file_key)?;
    Prompts::load(prompt_config_path).await
}

#[derive(Debug, Deserialize)]
pub struct NodeInfo {
    prompt_id: u64,
    version: String,
}
pub async fn create_node(
    State(data): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
    Json(payload): Json<NodeInfo>,
) -> AppResponse<CreateResponse> {
    let mut prompt_config = match find_prompt(&data.sql_conn, claims.id, payload.prompt_id).await {
        Ok(p) => p,
        Err(e) => return AppResponse::internal_err(format!("Failed to find prompt: {}", e)),
    };
    if let Err(e) = prompt_config.create_version(&payload.version).await {
        return AppResponse::internal_err(format!("Failed to create version: {}", e));
    }
    if let Err(e) = prompt_config.save().await {
        return AppResponse::internal_err(format!("Failed to save prompt config: {}", e));
    }

    AppResponse::ok(
        format!("Create node version {} finished", payload.version),
        None,
    )
}

#[derive(Debug, Deserialize)]
pub struct CommitInfo {
    prompt_id: u64,
    version: String,
    desp: String,
    content: String,
}

#[derive(Debug, Serialize)]
pub struct CommitResponse {
    commit_id: String,
}
pub async fn create_commit(
    State(data): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
    Json(payload): Json<CommitInfo>,
) -> AppResponse<CommitResponse> {
    let mut prompt_config = match find_prompt(&data.sql_conn, claims.id, payload.prompt_id).await {
        Ok(p) => p,
        Err(e) => return AppResponse::internal_err(format!("Failed to find prompt: {}", e)),
    };
    if let Err(e) = prompt_config.save().await {
        return AppResponse::internal_err(format!("Failed to save prompt config: {}", e));
    }
    let commit = PromptCommit::new(claims.email, payload.desp);
    if let Err(e) = prompt_config
        .commit(&payload.version, commit.clone(), &payload.content)
        .await
    {
        return AppResponse::internal_err(format!("Failed to commit prompt: {}", e));
    }

    AppResponse::ok(
        "Create commit finished".to_string(),
        Some(CommitResponse {
            commit_id: commit.commit_id,
        }),
    )
}
pub fn routes(app_state: Arc<AppState>) -> Router {
    let jwt_auth = JwtAuth {
        conf: Arc::new(app_state.config.jwt_conf.clone()),
    };
    Router::new()
        .route("/create_prompt", post(create_prompt))
        .route("/create_node", post(create_node))
        .route("/create_commit", post(create_commit))
        .layer(ValidateRequestHeaderLayer::custom(jwt_auth))
        .with_state(app_state)
}
