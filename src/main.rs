mod db;
mod init;
mod logger;
mod routes;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::time::{Duration, SystemTime};

use axum::{
    body::Body,
    http::{Request, Response},
};
use routes::common::START_TIME;
use tokio::net;
use tower_http::{
    catch_panic::CatchPanicLayer, classify::ServerErrorsFailureClass,
    timeout::RequestBodyTimeoutLayer, trace::TraceLayer,
};
use tracing::{Span, error, info};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let _guard = logger::init().await;
    let start_time = START_TIME.get_or_init(SystemTime::now);
    info!("server start at {:?}", start_time);
    let port = std::env::var("PORT").unwrap_or("8000".to_string());
    let listener = net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|_request: &Request<Body>| {
            let request_id = Uuid::new_v4().to_string();
            tracing::info_span!("http-request: ", %request_id)
        })
        .on_request(|request: &Request<Body>, _span: &Span| {
            info!("request: {} {}", request.method(), request.uri().path())
        })
        .on_response(
            |response: &Response<Body>, latency: Duration, _span: &Span| {
                info!("response: {} {:?}", response.status(), latency);
            },
        )
        .on_failure(
            |err: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                error!("Don't panic, C'est la vie. {}", err)
            },
        );

    let app = routes::routes()
        .await
        .layer(trace_layer)
        .layer(CatchPanicLayer::new())
        .layer(RequestBodyTimeoutLayer::new(Duration::from_secs(30)));
    axum::serve(listener, app).await.unwrap();
}
