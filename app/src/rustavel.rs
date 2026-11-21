use std::sync::Arc;

mod helper;
mod http;
mod models;
mod routes;

use crate::helper::http_functions::{ROUTE_MANAGER, register_route_function};
use crate::http::middleware::error_page_middleware::error_pages_middleware;
use minijinja::{Environment, path_loader};
use rustavel_core::config::CONFIG;
use rustavel_core::config::database::create_db_pool;
use rustavel_core::logger;
use rustavel_core::mvc::view::{register_csrf_functions, register_global_func, resolve_views_dir};
use rustavel_core::route::http_middleware;
use rustavel_core::route::route_manager::RouteManager;
use rustavel_core::state::AppState;

#[tokio::main]
async fn main() {
    // APP_VIEWS_PATH for deployed binaries; the source tree otherwise
    let views_dir = resolve_views_dir().expect("Could not resolve views directory");

    let mut env = Environment::new();
    env.set_loader(path_loader(views_dir));

    register_global_func(&mut env);
    register_route_function(&mut env);
    register_csrf_functions(&mut env);

    let db_pool = create_db_pool().await.expect("Could not create db pool");

    let state = AppState {
        templates: Arc::new(env),
        db_pool,
    };

    let web = routes::web::web_routes(state.clone());
    let api = routes::api::api_routes(state.clone());

    let built = web.merge(api).unwrap_or_else(|e| {
        eprintln!("{:?}", e);
        std::process::exit(1);
    });

    let route_manager = Arc::new(RouteManager::new(built.names));

    ROUTE_MANAGER
        .set(Arc::clone(&route_manager))
        .expect("RouteManager already initialized");

    // take type annotation

    let app = http_middleware::apply(built.router, &CONFIG.http)
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            error_pages_middleware,
        ))
        .with_state(state);

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
