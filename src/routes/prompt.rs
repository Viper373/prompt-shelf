pub fn routes(app_state: Arc<AppState>) -> Router {
    let jwt_auth = JwtAuth {
        conf: Arc::new(app_state.config.jwt_conf.clone()),
    };
    Router::new()
        .route("/signin", post(sign_in))
        .route("/signup", post(sign_up))
        .with_state(app_state)
}
