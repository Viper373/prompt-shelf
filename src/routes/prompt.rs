use futures::{StreamExt, stream};
use std::sync::Arc;

use crate::db::prompts::{self, Entity as PromptData};
use anyhow::{Result, anyhow};
use axum::{
    Extension, Json, Router,
    extract::{Query, State},
    routing::{delete, get, post},
};
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait, ModelTrait,
    QueryFilter,
};
use serde::{Deserialize, Serialize};
use tower_http::validate_request::ValidateRequestHeaderLayer;
use tracing::{error, info};

use super::{
    common::{AppResponse, AppState, MAX_CONCURRENT_TASKS, PromptCommit, Prompts},
    finder::find_config,
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
        Err(e) => AppResponse::internal_err(format!("Failed to add prompt: {e}")),
    }
}

pub async fn query_prompt(
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

pub async fn query_latest_prompt(conn: &DatabaseConnection, user_id: i64) -> Result<PromptCommit> {
    let prompt = match PromptData::find()
        .filter(prompts::Column::UserId.eq(user_id))
        .one(conn)
        .await
    {
        Ok(Some(p)) => p,
        Ok(None) => return Err(anyhow!("Prompt id not exist!")),
        Err(e) => return Err(anyhow!("Failed to query db: {e}")),
    };
    if prompt.latest_version.is_none() || prompt.latest_commit.is_none() {
        return Err(anyhow!("Invalid prompt commit/version "));
    }
    let prompt_config_path = find_config(&prompt.file_key)?;
    let prompt_config = Prompts::load(prompt_config_path).await?;
    prompt_config
        .get_commit(
            &prompt.latest_version.unwrap(),
            &prompt.latest_commit.unwrap(),
        )
        .await
}

pub async fn delete_prompt(conn: &DatabaseConnection, user_id: i64, prompt_id: u64) -> Result<()> {
    let prompt = match PromptData::find_by_id(prompt_id)
        .filter(prompts::Column::UserId.eq(user_id))
        .one(conn)
        .await
    {
        Ok(Some(p)) => p,
        Ok(None) => return Err(anyhow!("Prompt id not exist!")),
        Err(e) => return Err(anyhow!("Failed to query db: {}", e)),
    };
    Prompts::delete(&prompt.file_key).await?;
    let _ = prompt
        .delete(conn)
        .await
        .map_err(|e| anyhow!("Failed to delete prompt: {e}"));
    Ok(())
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
    let mut prompt_config = match query_prompt(&data.sql_conn, claims.id, payload.prompt_id).await {
        Ok(p) => p,
        Err(e) => return AppResponse::internal_err(format!("Failed to find prompt: {e}")),
    };
    if let Err(e) = prompt_config.create_version(&payload.version).await {
        return AppResponse::internal_err(format!("Failed to create version: {e}"));
    }
    if let Err(e) = prompt_config.save().await {
        return AppResponse::internal_err(format!("Failed to save prompt config: {e}"));
    }
    if let Err(e) = PromptData::update(prompts::ActiveModel {
        latest_version: Set(Some(payload.version.clone())),
        ..Default::default()
    })
    .exec(&data.sql_conn)
    .await
    {
        return AppResponse::internal_err(format!("Failed to update prompt version: {e}"));
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
    let mut prompt_config = match query_prompt(&data.sql_conn, claims.id, payload.prompt_id).await {
        Ok(p) => p,
        Err(e) => return AppResponse::internal_err(format!("Failed to find prompt: {e}")),
    };
    if let Err(e) = prompt_config.save().await {
        return AppResponse::internal_err(format!("Failed to save prompt config: {e}"));
    }
    let commit = PromptCommit::new(claims.email, payload.desp);
    if let Err(e) = prompt_config
        .commit(&payload.version, commit.clone(), &payload.content)
        .await
    {
        return AppResponse::internal_err(format!("Failed to commit prompt: {e}"));
    }
    if let Err(e) = PromptData::update(prompts::ActiveModel {
        latest_commit: Set(Some(commit.commit_id.clone())),
        ..Default::default()
    })
    .exec(&data.sql_conn)
    .await
    {
        return AppResponse::internal_err(format!("Failed to update prompt version: {e}"));
    }

    AppResponse::ok(
        "Create commit finished".to_string(),
        Some(CommitResponse {
            commit_id: commit.commit_id,
        }),
    )
}

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    id: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct PromptResponse {
    id: u64,
    latest_version: Option<String>,
    latest_commit: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    user_id: Option<i64>,
    org_id: Option<i64>,
    prompt: Prompts,
}

pub async fn query(
    State(data): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
    Query(params): Query<QueryParams>,
) -> AppResponse<Vec<PromptResponse>> {
    let mut filter_condition = Condition::all();
    if let Some(prompt_id) = params.id {
        filter_condition = filter_condition.add(prompts::Column::Id.eq(prompt_id));
    }
    let prompt_list = match PromptData::find()
        .filter(filter_condition)
        .filter(prompts::Column::UserId.eq(claims.id))
        .all(&data.sql_conn)
        .await
    {
        Ok(p) => p,
        Err(e) => return AppResponse::internal_err(format!("Failed to query db: {e}")),
    };

    let res: Vec<PromptResponse> = stream::iter(prompt_list)
        .map(|p| async move {
            let prompt_config_path = match find_config(&p.file_key) {
                Ok(p) => p,
                Err(e) => {
                    error!("Failed to find prompt: {e}");
                    return None;
                }
            };
            let prompt = match Prompts::load(prompt_config_path).await {
                Ok(p) => p,
                Err(e) => {
                    error!("Failed to load prompt: {e}");
                    return None;
                }
            };
            Some(PromptResponse {
                id: p.id,
                latest_version: p.latest_version.clone(),
                latest_commit: p.latest_commit.clone(),
                created_at: p.created_at,
                updated_at: p.updated_at,
                user_id: p.user_id,
                org_id: p.org_id,
                prompt,
            })
        })
        .buffer_unordered(MAX_CONCURRENT_TASKS)
        .filter_map(|p| async move { p })
        .collect()
        .await;

    AppResponse::ok("Query prompt finished".to_string(), Some(res))
}

pub async fn latest(
    State(data): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
) -> AppResponse<PromptCommit> {
    match query_latest_prompt(&data.sql_conn, claims.id).await {
        Ok(c) => AppResponse::ok("Query successfully".to_string(), Some(c)),
        Err(e) => AppResponse::internal_err(format!("Query failed: {e}")),
    }
}

pub async fn del(
    State(data): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
    Query(params): Query<QueryParams>,
) -> AppResponse<CreateResponse> {
    if let Some(id) = params.id {
        match delete_prompt(&data.sql_conn, claims.id, id).await {
            Ok(()) => AppResponse::ok(
                "prompt has been deleted".to_string(),
                Some(CreateResponse { id }),
            ),
            Err(e) => AppResponse::internal_err(format!("failed to delete prompt: {e}")),
        }
    } else {
        AppResponse::bad_request("prompt id is null")
    }
}

pub fn routes(app_state: Arc<AppState>) -> Router {
    let jwt_auth = JwtAuth {
        conf: Arc::new(app_state.config.jwt_conf.clone()),
    };
    Router::new()
        .route("/create_prompt", post(create_prompt))
        .route("/create_node", post(create_node))
        .route("/create_commit", post(create_commit))
        .route("/query", get(query))
        .route("/latest", get(latest))
        .route("/", delete(del))
        .layer(ValidateRequestHeaderLayer::custom(jwt_auth))
        .with_state(app_state)
}
