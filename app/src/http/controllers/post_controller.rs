// main model
use crate::models::Post;

use crate::helper::paginator::PaginateBuilder;
use crate::models::comment::CommentRow;
use crate::models::post::{PostResource, PostRow, PostsResource};
use crate::models::user::UserSimple;
use axum::Json;
use axum::extract::{OriginalUri, Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use rustavel_core::config::database::DbPoolType;
use serde::Deserialize;
use sqlx::AssertSqlSafe;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<usize>,
    pub max: Option<usize>,
    pub count: Option<usize>,
}

pub async fn index(
    State(pool): State<DbPoolType>,
    OriginalUri(uri): OriginalUri,
    Query(query): Query<PaginationQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(1).max(1);
    let max = query.max.or(query.count).unwrap_or(10).clamp(1, 100);

    // Total count
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts")
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    let count = count as usize;

    // Pagination offset
    let offset = (page - 1) * max;

    // Fetch posts

    let posts: Vec<PostRow> = sqlx::query_as(AssertSqlSafe(format!(
        r#"
        SELECT {}
        FROM posts
        ORDER BY id DESC
        LIMIT ? OFFSET ?
    "#,
        PostRow::COLUMNS
    )))
    .bind(max as i64)
    .bind(offset as i64)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let data = posts
        .into_iter()
        .map(PostsResource::from)
        .collect::<Vec<_>>();

    // Build pagination response
    let pagination = PaginateBuilder::new(max, count, page, uri.to_string()).build(data);

    Json(pagination)
}

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

    let (user, comments) = match tokio::try_join!(
        sqlx::query_as::<_, UserSimple>(AssertSqlSafe(format!(
            "SELECT {} FROM users WHERE id = ?",
            UserSimple::COLUMNS
        )))
        .bind(post.user_id)
        .fetch_one(&pool),
        sqlx::query_as::<_, CommentRow>(AssertSqlSafe(format!(
            r#"
        SELECT {}
        FROM comments
        WHERE commentable_type = 'Model::Post'
        AND commentable_id = ?
    "#,
            CommentRow::COLUMNS
        )))
        .bind(post.id)
        .fetch_all(&pool),
    ) {
        Ok(result) => result,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let post = PostResource::from((post, user, comments));
    Json(post).into_response()
}
