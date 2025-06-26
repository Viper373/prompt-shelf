use std::{env, sync::Arc};

use axum::Router;
use common::AppState;
use config::Config;

use crate::init::{init_db, redis_pool};

pub mod common;
pub mod config;
pub mod finder;
pub mod status;

pub async fn routes() -> Router {
    let sql_uri =
        env::var("SQL_URI").unwrap_or("mysql://admin:prompt-shelf@mysql:3306/aigame".to_string());
    let sql_conn = init_db(&sql_uri).await.unwrap();
    let redis_uri =
        env::var("SQL_URI").unwrap_or("redis://:promptshelf-25@dragonfly:6379".to_string());
    let redis_pool = redis_pool(&redis_uri).await.unwrap();
    let config = Config::from_env();
    let app_state = Arc::new(AppState {
        sql_conn,
        config,
        redis_pool,
    });
    Router::new().nest("/status", status::routes())
}
