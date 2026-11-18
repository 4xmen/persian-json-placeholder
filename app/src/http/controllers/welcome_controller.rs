use axum::http::StatusCode;
use axum::{Extension, Json, extract::State, response::IntoResponse};
use std::time::Instant;
// use minijinja::functions::debug;
use rustavel_core::config::CONFIG;
use rustavel_core::config::database::DbPoolType;
use rustavel_core::helper::datetime::format_duration;
use rustavel_core::state::AppState;
use serde::Serialize;

#[derive(Serialize)]
struct SampleData {
    title: String,
    elapsed_time: Option<String>,
    url: String,
}

#[derive(Serialize, Debug)]
pub struct WelcomeResponse {
    pub app_name: String,
    pub api_version: String,

    pub entities: Vec<EntityCount>,
    pub urls: Vec<ApiUrl>,
}

#[derive(Serialize, Debug)]
pub struct ApiUrl {
    pub name: String,
    pub url: String,
}
#[derive(Serialize, Debug)]
pub struct EntityCount {
    pub entity: String,
    pub count: i64,
}

/// Renders the welcome page using MiniJinja.
pub async fn index(
    State(state): State<AppState>,
    Extension(start): Extension<Instant>,
) -> impl IntoResponse {
    // Prepare the data passed to the template.

    let elapsed_ms = format_duration(start.elapsed());

    let sample = SampleData {
        title: CONFIG.app.name.clone(),
        elapsed_time: Some(elapsed_ms),
        url: CONFIG.app.url.clone(),
    };

    state.view_with("welcome.j2", sample)
}

pub async fn api(State(pool): State<DbPoolType>) -> impl IntoResponse {
    let sql_products = "SELECT COUNT(*) FROM products";
    let sql_categories = "SELECT COUNT(*) FROM categories";
    let sql_posts = "SELECT COUNT(*) FROM posts";
    let sql_todos = "SELECT COUNT(*) FROM todos";
    let sql_comments = "SELECT COUNT(*) FROM comments";
    let sql_user = "SELECT COUNT(*) FROM users";

    let (product_count, category_count, post_count, todo_count, comment_count, user_count) = match tokio::try_join!(
        sqlx::query_scalar::<_, i64>(sql_products).fetch_one(&pool),
        sqlx::query_scalar::<_, i64>(sql_categories).fetch_one(&pool),
        sqlx::query_scalar::<_, i64>(sql_posts).fetch_one(&pool),
        sqlx::query_scalar::<_, i64>(sql_todos).fetch_one(&pool),
        sqlx::query_scalar::<_, i64>(sql_comments).fetch_one(&pool),
        sqlx::query_scalar::<_, i64>(sql_user).fetch_one(&pool),
    ) {
        Ok(result) => result,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let base_url = format!("{}/api/v1", CONFIG.app.url);
    let response = WelcomeResponse {
        app_name: "Persian JSONPlaceholder by Rustavel".to_string(),
        api_version: "v1".to_string(),

        entities: vec![
            EntityCount {
                entity: "Product".to_string(),
                count: product_count,
            },
            EntityCount {
                entity: "Post".to_string(),
                count: post_count,
            },
            EntityCount {
                entity: "Category".to_string(),
                count: category_count,
            },
            EntityCount {
                entity: "User".to_string(),
                count: user_count,
            },
            EntityCount {
                entity: "Todo".to_string(),
                count: todo_count,
            },
            EntityCount {
                entity: "Comment".to_string(),
                count: comment_count,
            },
        ],

        urls: vec![
            ApiUrl {
                name: "Posts".to_string(),
                url: format!("{}/posts", base_url),
            },
            ApiUrl {
                name: "Products".to_string(),
                url: format!("{}/products", base_url),
            },
            ApiUrl {
                name: "Categories".to_string(),
                url: format!("{}/categories", base_url),
            },
            ApiUrl {
                name: "Users".to_string(),
                url: format!("{}/users", base_url),
            },
            ApiUrl {
                name: "Todo".to_string(),
                url: format!("{}/todos", base_url),
            },
        ],
    };

    Json(response).into_response()
}
