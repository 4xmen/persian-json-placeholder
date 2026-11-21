use crate::routes;
use rustavel_core::route::route_manager::RouteManager;
use std::sync::{Arc, LazyLock};

pub static ROUTE_MANAGER: LazyLock<Arc<RouteManager>> = LazyLock::new(|| {
    let web = routes::web::web_routes();
    let api = routes::api::api_routes();

    let built = web.merge(api).unwrap_or_else(|e| {
        panic!("Failed to merge routes: {e:?}");
    });

    Arc::new(RouteManager::new(built.names))
});
