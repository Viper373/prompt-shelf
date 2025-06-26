// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   status.rs                                          :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: dfine <coding@dfine.tech>                  +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2025/06/10 17:35:33 by dfine             #+#    #+#             //
//   Updated: 2025/06/10 17:35:33 by dfine            ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use std::time::SystemTime;

use axum::{Json, Router, routing::get};
use serde::Serialize;

use super::common::START_TIME;

#[derive(Serialize)]
pub struct Health {
    status: &'static str,
    uptime_seconds: u64,
}

pub async fn status() -> Json<Health> {
    let start_time = START_TIME.get_or_init(SystemTime::now);
    Json(Health {
        status: "Ok",
        uptime_seconds: SystemTime::now()
            .duration_since(*start_time)
            .unwrap()
            .as_secs(),
    })
}

pub fn routes() -> Router {
    Router::new().route("/", get(status))
}
