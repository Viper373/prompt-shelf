// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   config.rs                                          :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: dfine <coding@dfine.tech>                  +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2025/06/10 17:35:15 by dfine             #+#    #+#             //
//   Updated: 2025/06/25 17:46:47 by dfine            ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use std::{env, sync::atomic::AtomicBool};

use super::middleware::JwtConf;

#[derive(Debug, Clone)]
pub struct Config {
    pub data_dir: String,
    pub jwt_conf: JwtConf,
}

impl Config {
    pub fn from_env() -> Self {
        let data_dir = env::var("DATA_DIR").unwrap_or("/data".to_string());
        let secret = env::var("JWT_SECRET").unwrap_or("promptshelf".to_string());
        let expire = env::var("JWT_EXPIRE")
            .unwrap_or("168".to_string())
            .parse::<i64>()
            .unwrap();

        Config {
            data_dir,
            jwt_conf: JwtConf { secret, expire },
        }
    }
}
