// main model
use crate::models::{Comment, Post};

use axum::extract::{OriginalUri, Path, Query, RawPathParams, RawQuery, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use rustavel_core::config::CONFIG;
use rustavel_core::config::database::DbPoolType;
use rustavel_core::state::AppState;
use crate::helper::general::{asset_url, PaginateBuilder};
use crate::models::post::PostResponse;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<usize>,
    pub max: Option<usize>,
}


#[derive(Serialize, Debug)]
pub struct SinglePostResponse {
    pub id: u64,
    pub title: String,
    pub slug: String,
    pub body: Option<String>,
    pub image: Option<String>,
    pub created_at: Option<time::OffsetDateTime>,
    pub updated_at: Option<time::OffsetDateTime>,
    pub url: String,
    pub user: UserResponse,
}

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct UserResponse {
    pub id: u64,
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct PostShowResponse {
    pub post: SinglePostResponse,
    pub comments: Vec<Comment>,
}

pub async fn index(
    State(pool): State<DbPoolType>,
    OriginalUri(uri): OriginalUri,
    Query(query): Query<PaginationQuery>,
) -> impl IntoResponse {

    let page = query.page.unwrap_or(1).max(1);
    let max = query.max.unwrap_or(10).clamp(1, 100);

    // Total count
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM posts"
    )
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    let count = count as usize;

    // Pagination offset
    let offset = (page - 1) * max;

    // Fetch posts
    let sql = r#"
        SELECT *
        FROM posts
        ORDER BY id DESC
        LIMIT ? OFFSET ?
    "#;

    let posts: Vec<Post> = sqlx::query_as(sql)
        .bind(max as i64)
        .bind(offset as i64)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    // Convert Post -> PostResponse
    let data: Vec<PostResponse> =posts
        .into_iter()
        .map(|post| PostResponse::from((post, format!("{}/api/v1/posts", CONFIG.app.url))))
        .collect();

    // Build pagination response
    let pagination = PaginateBuilder::new(
        max,
        count,
        page,
        uri.to_string(),
    ).build(data);

    Json(pagination)
}

// pub async fn create(State(_state): State<AppState>) -> impl IntoResponse {
//     (StatusCode::OK, "to create Post called")
// }
//
// pub async fn store(State(_state): State<AppState>, params: RawPathParams) -> impl IntoResponse {
//     (
//         StatusCode::OK,
//         println!("to store Post called request: {:?}", params),
//     )
// }
//
// pub async fn edit(State(_state): State<AppState>, query: RawQuery) -> impl IntoResponse {
//     (
//         StatusCode::OK,
//         println!("to edit Post called id: {:?}", query),
//     )
// }

// pub async fn update(
//     State(_state): State<AppState>,
//     query: RawQuery,
//     params: RawPathParams,
// ) -> impl IntoResponse {
//     (
//         StatusCode::OK,
//         println!(
//             "to edit Post called id and request: {:?}, {:?}",
//             query, params
//         ),
//     )
// }

pub async fn show(
    State(pool): State<DbPoolType>,
    Path(slug): Path<String>,
) -> impl IntoResponse {

    let sql_post = "SELECT * FROM posts WHERE slug = ?";

    let post = match sqlx::query_as::<_, Post>(sql_post)
        .bind(&slug)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(post)) => post,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let sql_user = "SELECT id, name, email FROM users WHERE id = ?";

    let sql_comment = r#"
        SELECT *
        FROM comments
        WHERE commentable_type = 'Model::Post'
        AND commentable_id = ?
    "#;

    let (user, comments) = match tokio::try_join!(
        sqlx::query_as::<_, UserResponse>(sql_user)
            .bind(post.user_id)
            .fetch_one(&pool),

        sqlx::query_as::<_, Comment>(sql_comment)
            .bind(post.id)
            .fetch_all(&pool),
    ) {
        Ok(result) => result,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let post_response = SinglePostResponse {
        id: post.id,
        title: post.title,
        slug: post.slug.clone(),
        body: post.body,
        image: asset_url(&Some(post.image)),
        created_at: post.created_at,
        updated_at: post.updated_at,
        url: format!("{}/api/v1/posts",CONFIG.app.url),
        user,
    };

    Json(PostShowResponse {
        post: post_response,
        comments,
    }).into_response()
}

// pub async fn destroy(State(_state): State<AppState>, query: RawQuery) -> impl IntoResponse {
//     (
//         StatusCode::OK,
//         println!("to destroy Post called id: {:?}", query),
//     )
// }
