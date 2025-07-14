use std::sync::Arc;

use crate::db::prompts::{self, Entity as PromptData};
use axum::{Extension, Json, Router, extract::State, routing::post};
use sea_orm::{ActiveValue::Set, EntityTrait};
use serde::{Deserialize, Serialize};
use tower_http::validate_request::ValidateRequestHeaderLayer;
use tracing::{error, info};

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

pub fn routes(app_state: Arc<AppState>) -> Router {
    let jwt_auth = JwtAuth {
        conf: Arc::new(app_state.config.jwt_conf.clone()),
    };
    Router::new()
        .route("/create_prompt", post(create_prompt))
        .layer(ValidateRequestHeaderLayer::custom(jwt_auth))
        .with_state(app_state)
}
