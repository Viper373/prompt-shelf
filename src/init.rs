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
use deadpool_redis::{self, Pool};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
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

pub async fn redis_pool(uri: &str) -> Result<Pool, Box<dyn std::error::Error>> {
    let cfg = deadpool_redis::Config::from_url(uri);
    let pool = cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1))?;
    Ok(pool)
}
