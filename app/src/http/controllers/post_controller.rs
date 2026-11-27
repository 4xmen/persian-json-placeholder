// main model
use crate::models::{Comment, Post, User};

use crate::helper::general::{PaginateBuilder, asset_url};
use crate::models::post::{PostResource, PostsResource};
use axum::Json;
use axum::extract::{OriginalUri, Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use rustavel_core::config::CONFIG;
use rustavel_core::config::database::DbPoolType;
use serde::{Deserialize, Serialize};
use crate::models::user::UserResource;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<usize>,
    pub max: Option<usize>,
}


pub async fn index(
    State(pool): State<DbPoolType>,
    OriginalUri(uri): OriginalUri,
    Query(query): Query<PaginationQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(1).max(1);
    let max = query.max.unwrap_or(10).clamp(1, 100);

    // Total count
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts")
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    let count = count as usize;

    // Pagination offset
    let offset = (page - 1) * max;

    // Fetch posts
    let sql = r#"
        SELECT id,title,slug,image,created_at,updated_at,body
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
    // let data = PostsResource::collection(posts);

    // Build pagination response
    // let pagination = PaginateBuilder::new(max, count, page, uri.to_string()).build_resource(data);

    Json(posts)
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

pub async fn show(State(pool): State<DbPoolType>, Path(slug): Path<String>) -> impl IntoResponse {
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
        sqlx::query_as::<_, User>(sql_user)
            .bind(post.user_id)
            .fetch_one(&pool),
        sqlx::query_as::<_, Comment>(sql_comment)
            .bind(post.id)
            .fetch_all(&pool),
    ) {
        Ok(result) => result,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    // let post_response = SinglePostResponse {
    //     id: post.id,
    //     title: post.title,
    //     slug: post.slug.clone(),
    //     body: post.body,
    //     image: asset_url(&Some(post.image)),
    //     created_at: post.created_at,
    //     updated_at: post.updated_at,
    //     url: format!("{}/api/v1/posts", CONFIG.app.url),
    //     user,
    // };
    //
    // Json(PostShowResponse {
    //     post: post_response,
    //     comments,
    // })
    // .into_response()
    Json({}).into_response()
}

// pub async fn destroy(State(_state): State<AppState>, query: RawQuery) -> impl IntoResponse {
//     (
//         StatusCode::OK,
//         println!("to destroy Post called id: {:?}", query),
//     )
// }
