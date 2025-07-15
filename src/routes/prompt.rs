use std::sync::Arc;

use crate::db::prompts::{self, Entity as PromptData};
use axum::{Extension, Json, Router, extract::State, routing::post};
use sea_orm::{ActiveValue::Set, EntityTrait};
use sea_orm::{ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use tower_http::validate_request::ValidateRequestHeaderLayer;
use tracing::info;

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

#[axum::debug_handler]
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
    let prompt = match PromptData::find_by_id(payload.prompt_id)
        .filter(prompts::Column::UserId.eq(claims.id))
        .one(&data.sql_conn)
        .await
    {
        Ok(Some(p)) => p,
        Ok(None) => return AppResponse::bad_request("Prompt id not exist!".to_string()),
        Err(e) => return AppResponse::internal_err(format!("Failed to query db: {}", e)),
    };
    let prompt_config_path = match find_config(&prompt.file_key) {
        Ok(p) => p,
        Err(e) => return AppResponse::internal_err(format!("Failed to find prompt: {}", e)),
    };
    let mut prompt_config = match Prompts::load(prompt_config_path).await {
        Ok(p) => p,
        Err(e) => return AppResponse::internal_err(format!("Failed to load prompt: {}", e)),
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

pub fn routes(app_state: Arc<AppState>) -> Router {
    let jwt_auth = JwtAuth {
        conf: Arc::new(app_state.config.jwt_conf.clone()),
    };
    Router::new()
        .route("/create_prompt", post(create_prompt))
        .route("/create_node", post(create_node))
        .layer(ValidateRequestHeaderLayer::custom(jwt_auth))
        .with_state(app_state)
}
