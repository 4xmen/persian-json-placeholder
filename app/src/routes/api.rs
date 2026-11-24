use crate::http::controllers::test_controller::register;
use crate::http::controllers::welcome_controller::api;
use crate::http::controllers::{
    category_controller, post_controller, product_controller, todo_controller, user_controller,
};
use rustavel_core::route::routing::{BuiltRoutes, Route};
use rustavel_core::state::AppState;
// import AppState

pub fn api_routes(_app_state: AppState) -> BuiltRoutes<AppState> {
    let mut route = Route::new();

    route.group(|r| {
        r.name("api").prefix("/api");
        // .middleware(log_middleware::log_request);
        r.get("", api).name("welcome");

        r.group(|v1| {
            v1.prefix("/v1").name("v1");
            v1.get("", api).name("index");
            v1.group(|users| {
                users.prefix("/users").name("users");
                users.get("", user_controller::index).name("index");
                users.get("/{id}", user_controller::show).name("show");
            });
            v1.group(|posts| {
                posts.prefix("/posts").name("posts");
                posts.get("", post_controller::index).name("index");
                posts.get("/{slug}", post_controller::show).name("show");
            });
            v1.group(|products| {
                products.prefix("/products").name("products");
                products.get("", product_controller::index).name("index");
                products
                    .get("/{slug}", product_controller::show)
                    .name("show");
            });
            v1.group(|todos| {
                todos.prefix("/todos").name("todos");
                todos.get("", todo_controller::index).name("index");
                // todos.get("/{id}", todo_controller::show).name("show");
            });
            v1.group(|cats| {
                cats.prefix("/categories").name("categories");
                cats.get("", category_controller::index).name("index");
                cats.get("/{slug}", category_controller::show).name("show");
            });
            v1.post("register", register).name("register");
        })
    });

    route.build().unwrap_or_else(|e| {
        eprintln!("{:?}", e);
        std::process::exit(1);
    })
}

// async fn hello_api() -> impl IntoResponse {
//     (StatusCode::OK, "hello api")
// }
