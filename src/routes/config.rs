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

use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub data_dir: String,
}

impl Config {
    pub fn from_env() -> Self {
        let data_dir = env::var("DATA_DIR").unwrap_or("/data".to_string());
        Config { data_dir }
    }
}
