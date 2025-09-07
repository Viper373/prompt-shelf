// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   init.rs                                            :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: dfine <coding@dfine.tech>                  +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2025/03/20 20:09:33 by dfine             #+#    #+#             //
//   Updated: 2025/04/15 20:42:54 by dfine            ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use anyhow::{Context, Result, anyhow};
use deadpool_redis::redis::AsyncCommands;
use deadpool_redis::{self, Pool};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbBackend, Schema, ConnectionTrait, Statement};
use tokio::time::Duration;
use tracing::*;

pub async fn init_db(uri: &str) -> Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(uri);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(30))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(30))
        .max_lifetime(Duration::from_secs(30))
        .sqlx_logging(false)
        .sqlx_logging_level(log::LevelFilter::Info);
    Database::connect(opt)
        .await
        .map_err(|e| anyhow!(e))
        .context("Failed to connect to the database")
}

pub async fn ensure_tables(conn: &DatabaseConnection) -> Result<()> {
    let backend: DbBackend = conn.get_database_backend();

    // users
    let users_sql = r#"
CREATE TABLE IF NOT EXISTS users (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  username VARCHAR(100) NOT NULL UNIQUE,
  email VARCHAR(255) NOT NULL UNIQUE,
  role VARCHAR(16) NOT NULL DEFAULT 'user',
  valid BOOLEAN NOT NULL DEFAULT TRUE,
  password_hash VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
)
"#;

    // prompts
    let prompts_sql = r#"
CREATE TABLE IF NOT EXISTS prompts (
  id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
  latest_version VARCHAR(32),
  latest_commit VARCHAR(64),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  user_id BIGINT,
  file_key VARCHAR(100) NOT NULL,
  org_id BIGINT
)
"#;

    // organizations
    let orgs_sql = r#"
CREATE TABLE IF NOT EXISTS organizations (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(255) NOT NULL UNIQUE,
  admin_id BIGINT NOT NULL,
  description TEXT,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
)
"#;

    // user_organizations
    let map_sql = r#"
CREATE TABLE IF NOT EXISTS user_organizations (
  user_id BIGINT NOT NULL,
  org_id BIGINT NOT NULL,
  PRIMARY KEY (user_id, org_id),
  INDEX idx_org_id (org_id)
)
"#;

    for sql in [users_sql, prompts_sql, orgs_sql, map_sql] {
        conn.execute(Statement::from_string(backend, sql.to_string())).await?;
    }

    Ok(())
}

pub async fn redis_pool(uri: &str) -> Result<Pool, Box<dyn std::error::Error>> {
    let cfg = deadpool_redis::Config::from_url(uri);
    let pool = cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1))?;
    Ok(pool)
}

pub async fn get_cache(key: &str, conn: &mut deadpool_redis::Connection) -> Result<String> {
    let value: String = match conn.get(key).await {
        Ok(Some(v)) => v,
        Ok(None) => return Err(anyhow!("Value not exist")),
        Err(e) => return Err(anyhow!("Failed to query db: {e}")),
    };
    Ok(value)
}

pub async fn set_cache(
    key: &str,
    value: &str,
    expire_secs: Option<u64>,
    conn: &mut deadpool_redis::Connection,
) -> Result<()> {
    if let Some(exp) = expire_secs {
        conn.set_ex::<_, _, ()>(key, value, exp).await?;
    } else {
        conn.set::<_, _, ()>(key, value).await?;
    }
    Ok(())
}
