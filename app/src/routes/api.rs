
use crate::http::controllers::user_controller;
use crate::http::controllers::test_controller::register;
use crate::http::middleware::log_middleware;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use rustavel_core::route::routing::{BuiltRoutes, Route};
use rustavel_core::state::AppState; // import AppState

pub fn api_routes() -> BuiltRoutes<AppState> {
    let mut route = Route::new();

    route.group(|r| {
        r.name("api")
            .prefix("/api")
            .middleware(log_middleware::log_request);

        r.group(|v1| {
            v1.prefix("/v1").name("v1");
            v1.get("", hello_api).name("index");
            v1.group(|users| {
                users.prefix("/users").name("users");
                users.get("", user_controller::index).name("index");
                users.get("create", hello_api).name("create");
            });
            v1.post("register", register).name("register");
        })
    });

    route.build().unwrap_or_else(|e| {
        eprintln!("{:?}", e);
        std::process::exit(1);
    })
}

async fn hello_api() -> impl IntoResponse {
    (StatusCode::OK, "hello api")
}
