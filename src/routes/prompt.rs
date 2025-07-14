use std::sync::Arc;

use crate::db::prompts::{self, Entity as PromptData};
use axum::{Json, Router, extract::State};
use serde::{Deserialize, Serialize};

use super::{
    common::{AppResponse, AppState, Prompts},
    middleware::JwtAuth,
};

#[derive(Debug, Deserialize)]
pub struct PromptInfo {
    name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateResponse {
    id: i32,
}

pub async fn create_prompt(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<PromptInfo>,
) -> AppResponse<CreateResponse> {
    let prompt = Prompts::new(payload.name);
    todo!()
}

pub fn routes(app_state: Arc<AppState>) -> Router {
    let jwt_auth = JwtAuth {
        conf: Arc::new(app_state.config.jwt_conf.clone()),
    };
    Router::new().with_state(app_state)
}
