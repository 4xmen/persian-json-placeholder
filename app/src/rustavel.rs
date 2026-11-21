use std::sync::Arc;

mod helper;
mod http;
mod models;
mod routes;

use crate::helper::ROUTE_MANAGER;
use minijinja::{Environment, path_loader};
use rustavel_core::config::CONFIG;
use rustavel_core::config::database::create_db_pool;
use rustavel_core::logger;
use rustavel_core::mvc::view::{register_csrf_functions, register_global_func, resolve_views_dir};
use rustavel_core::route::http_middleware;
use rustavel_core::state::AppState;

#[tokio::main]
async fn main() {
    let web = routes::web::web_routes();
    let api = routes::api::api_routes();

    let built = web.merge(api).unwrap_or_else(|e| {
        eprintln!("{:?}", e);
        std::process::exit(1);
    });

    // APP_VIEWS_PATH for deployed binaries; the source tree otherwise
    let views_dir = resolve_views_dir().expect("Could not resolve views directory");

    let mut env = Environment::new();
    env.set_loader(path_loader(views_dir));

    register_global_func(&mut env, Arc::clone(&ROUTE_MANAGER));
    register_csrf_functions(&mut env);

    let db_pool = create_db_pool().await.expect("Could not create db pool");

    let state = AppState {
        templates: Arc::new(env),
        db_pool,
    };

    // take type annotation

    let app = http_middleware::apply(built.router, &CONFIG.http).with_state(state);

    //
    // let app: Router<AppState> = Router::new()
    //     .with_state(state)
    //     .route("/test", post(register));

    let app_start_point = format!("{}:{}", CONFIG.app.host, CONFIG.app.port);

    logger::info(&format!("Try server on http://{}", app_start_point));

    // let listener = tokio::net::TcpListener::bind(app_start_point.clone())
    //     .await
    //     .unwrap();

    let listener = tokio::net::TcpListener::bind(app_start_point.clone())
        .await
        .unwrap_or_else(|e| {
            eprintln!("can't listen on {}: {}", app_start_point, e);
            std::process::exit(1);
        });

    logger::success(&format!("Starting server on http://{}", app_start_point));
    axum::serve(listener, app).await.unwrap();
}
