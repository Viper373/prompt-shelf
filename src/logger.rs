// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   logger.rs                                          :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: dfine <coding@dfine.tech>                  +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2024/05/11 19:43:11 by dfine             #+#    #+#             //
//   Updated: 2025/03/20 20:09:06 by dfine            ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    Registry, filter::EnvFilter, fmt::Layer, fmt::format::Writer, fmt::time::FormatTime,
    layer::SubscriberExt, util::SubscriberInitExt,
};

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", chrono::Local::now().format("%FT%T%.3f"))
    }
}
fn clean() -> Result<(), std::io::Error> {
    let log_dir = "logs";
    std::fs::create_dir_all(log_dir)?;
    let retention_days = 7;

    for entry in std::fs::read_dir(log_dir)? {
        let path = entry?.path();
        if path.is_file() {
            let modified = path.metadata()?.modified()?;
            if chrono::Local::now() - chrono::DateTime::<chrono::Local>::from(modified)
                > chrono::Duration::days(retention_days)
            {
                std::fs::remove_file(path)?;
            }
        }
    }
    Ok(())
}
pub async fn init() -> Result<WorkerGuard, Box<dyn std::error::Error>> {
    clean()?;
    let (log_path, log_name) = ("logs", "aigame.log");
    let env_filter = EnvFilter::from_env("API_LOG");

    let file_appender = tracing_appender::rolling::daily(log_path, log_name);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let stdout_layer = Layer::default()
        .with_writer(std::io::stdout)
        .with_timer(LocalTimer)
        .with_ansi(true);
    let file_layer = Layer::default()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_timer(LocalTimer);
    Registry::default()
        .with(file_layer)
        .with(stdout_layer)
        .with(env_filter)
        .init();
    Ok(_guard)
}
