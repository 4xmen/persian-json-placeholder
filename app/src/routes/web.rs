use crate::http::controllers::test_controller;
use crate::http::controllers::todo_controller;
use crate::http::controllers::welcome_controller;
use crate::http::middleware::log_middleware;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use rustavel_core::route::routing::{BuiltRoutes, Route};

pub fn web_routes() -> BuiltRoutes<AppState> {
    let mut route = Route::new();

    route
        .get("/", welcome_controller::index)
        .name("welcome")
        .middleware(performance_middleware);
    route.get("/about", hello).name("about");
    // route.get("/about2",hello).name("about");
    route
        .any("/test", test_controller::register)
        .name("test")
        .middleware(log_middleware::log_request);

    route.group(|todo| {
        todo.name("todos").prefix("/todos");
        todo.get("", todo_controller::index).name("index");
        todo.get("/create", todo_controller::create).name("create");
        todo.post("", todo_controller::store).name("store");
        todo.get("edit/{id}", todo_controller::edit).name("edit");
        todo.get("show/{id}", todo_controller::show).name("show");
        todo.post("update/{id}", todo_controller::update)
            .name("update");
        todo.get("delete/{id}", todo_controller::destroy)
            .name("destroy");
    });

    route
        .get("/login", test_controller::login)
        .name("login.form");
    route
        .post("/login", test_controller::login_check)
        .name("login.check");

    let mut web = route.build().unwrap_or_else(|e| {
        eprintln!("{:?}", e);
        std::process::exit(1);
    });

    let sessions = SessionManager::new();

    web.router = web
        .router
        .merge(static_files::static_files())
        .layer(middleware::from_fn(csrf::verify))
        .layer(middleware::from_fn_with_state(sessions, session::layer));

    web
}

use crate::http::middleware::performance::performance_middleware;
use axum::extract::State;
use axum::middleware;
use rustavel_core::http::session::SessionManager;
use rustavel_core::http::{csrf, session, static_files};
use rustavel_core::state::AppState;
// import AppState

async fn hello(State(_state): State<AppState>) -> impl IntoResponse {
    // let welcome_url = state.route("welcome");
    // println!("Welcome route: {}", welcome_url);
    // let group_url = state.route("api.v1.users.create");
    // println!("Group route: {}", group_url);
    (StatusCode::OK, "hello world")
}
