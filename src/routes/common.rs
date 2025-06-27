// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   common.rs                                          :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: dfine <coding@dfine.tech>                  +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2025/06/10 17:35:22 by dfine             #+#    #+#             //
//   Updated: 2025/06/25 17:50:42 by dfine            ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use std::{path::Path, sync::OnceLock, time::SystemTime};

use anyhow::{Context, Ok, Result, anyhow};
use axum::{Json, http::StatusCode, response::IntoResponse};
use chrono::{DateTime, Utc};
use deadpool_redis::Pool;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::fs;
use uuid::Uuid;

use super::{config::Config, finder::find_prompt};

pub static START_TIME: OnceLock<SystemTime> = OnceLock::new();
pub struct AppState {
    pub sql_conn: DatabaseConnection,
    pub config: Config,
    pub redis_pool: Pool,
}

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum AppCode {
    Success = 200,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    InternalError = 500,
}

impl AppCode {
    pub fn http_status(self) -> StatusCode {
        match self {
            AppCode::Success => StatusCode::OK,
            AppCode::BadRequest => StatusCode::BAD_REQUEST,
            AppCode::Unauthorized => StatusCode::UNAUTHORIZED,
            AppCode::Forbidden => StatusCode::FORBIDDEN,
            AppCode::NotFound => StatusCode::NOT_FOUND,
            AppCode::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AppResponse<T: Serialize> {
    code: AppCode,
    msg: String,
    result: Option<T>,
}
impl<T: Serialize> IntoResponse for AppResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let res = Json(json!({"code":self.code, "msg":self.msg, "result":self.result}));
        (self.code.http_status(), res).into_response()
    }
}
impl<T: Serialize> AppResponse<T> {
    pub fn new(code: AppCode, msg: String, result: Option<T>) -> Self {
        Self { code, msg, result }
    }
    pub fn ok(msg: String, result: Option<T>) -> Self {
        Self {
            code: AppCode::Success,
            msg,
            result,
        }
    }
    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self {
            code: AppCode::BadRequest,
            msg: msg.into(),
            result: None,
        }
    }
    pub fn internal_err(msg: impl Into<String>) -> Self {
        Self {
            code: AppCode::InternalError,
            msg: msg.into(),
            result: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptCommit {
    pub author: String,
    pub commit_id: String,
    pub created_at: DateTime<Utc>,
    pub desp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptNode {
    pub version: String,
    pub commits: Vec<PromptCommit>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prompts {
    pub name: String,
    pub id: String,
    pub nodes: Vec<PromptNode>,
}

impl PromptCommit {
    pub fn new(author: String, desp: String) -> Self {
        Self {
            author,
            desp,
            commit_id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
        }
    }
}

impl Prompts {
    pub async fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path).await?;
        let data: Self = serde_json::from_str(&content)?;
        Ok(data)
    }
    pub async fn save<P: AsRef<Path>>(self, path: P) -> Result<()> {
        let content = serde_json::to_string_pretty(&self)?;
        fs::write(path, &content).await?;
        Ok(())
    }
    pub async fn commit(&mut self, version: &str, com: PromptCommit, content: &str) -> Result<()> {
        let save_path = find_prompt(&self.id, version, &com.commit_id)?;
        fs::write(save_path, content).await?;
        let node = self
            .nodes
            .iter_mut()
            .find(|n| n.version == version)
            .ok_or_else(|| anyhow!("Version {} not found!", version))?;
        node.commits.push(com);
        node.updated_at = Utc::now();
        Ok(())
    }
    pub async fn get_content(&self, version: &str, commit_id: &str) -> Result<String> {
        let save_path = find_prompt(&self.id, version, commit_id)?;
        let content = fs::read_to_string(save_path).await?;
        Ok(content)
    }
}
