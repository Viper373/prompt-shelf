use std::{env, sync::Arc};

use axum::Router;
use common::AppState;
use config::Config;

use crate::init::{init_db, redis_pool};

pub mod common;
pub mod config;
pub mod finder;
pub mod middleware;
pub mod prompt;
pub mod status;
pub mod user;

pub async fn routes() -> Router {
    let sql_uri = env::var("MYSQL_URI")
        .unwrap_or("mysql://shelf:shelf-25@mysql:3306/promptshelf".to_string());
    let sql_conn = init_db(&sql_uri).await.unwrap();
    let redis_uri =
        env::var("REDIS_URI").unwrap_or("redis://:promptshelf-25@dragonfly:6379".to_string());
    let redis_pool = redis_pool(&redis_uri).await.unwrap();
    let config = Config::from_env();
    let app_state = Arc::new(AppState {
        sql_conn,
        config,
        redis_pool,
    });
    Router::new()
        .nest("/status", status::routes())
        .nest("/user", user::routes(app_state.clone()))
}
