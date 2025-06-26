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
use axum::{http::StatusCode, response::IntoResponse};
use chrono::{DateTime, Utc};
use deadpool_redis::Pool;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tokio::fs;
use uuid::Uuid;

use super::{config::Config, finder::find_prompt};

pub static START_TIME: OnceLock<SystemTime> = OnceLock::new();
pub struct AppState {
    pub sql_conn: DatabaseConnection,
    pub config: Config,
    pub redis_pool: Pool,
}

#[derive(Debug)]
pub struct AppError {
    code: StatusCode,
    msg: String,
}
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.code, self.msg).into_response()
    }
}
impl AppError {
    pub fn new(code: StatusCode, msg: String) -> Self {
        Self { code, msg }
    }
    pub fn bad_response(msg: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, msg.into())
    }
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, msg.into())
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
        let save_path = find_prompt(&self.id, &version, &com.commit_id)?;
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
        let save_path = find_prompt(&self.id, &version, &commit_id)?;
        let content = fs::read_to_string(save_path).await?;
        Ok(content)
    }
}
