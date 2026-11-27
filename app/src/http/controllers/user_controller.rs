use std::fmt::format;
// main model
use crate::models::{Post, Product};

use crate::models::post::PostRow;
use crate::models::product::ProductRow;
use crate::models::user::{UserList, UserListResource, UserSingleResource};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use rustavel_core::config::database::DbPoolType;
use sqlx::AssertSqlSafe;

pub async fn index(State(pool): State<DbPoolType>) -> impl IntoResponse {
    let users = sqlx::query_as::<_, UserList>(AssertSqlSafe(format!(
        r#"
        SELECT {}
        FROM users
        WHERE deleted_at IS NULL
    "#,
        UserList::COLUMNS
    )))
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        println!("{:#?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
    .unwrap_or_default();

    Json(UserListResource::collection(users))
}

pub async fn show(
    State(pool): State<DbPoolType>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, StatusCode> {
    let (user, posts, products) = tokio::try_join!(
        async {
            sqlx::query_as::<_, UserList>(AssertSqlSafe(format!(
                r#"SELECT {} FROM users
        WHERE deleted_at IS NULL AND id = ? "#,
                UserList::COLUMNS,
            )))
            .bind(id)
            .fetch_optional(&pool)
            .await
            .map_err(|e| {
                println!("{:#?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .ok_or(StatusCode::NOT_FOUND)
        },
        async {
            sqlx::query_as::<_, PostRow>(AssertSqlSafe(format!(
                r#" SELECT {} FROM posts
        WHERE user_id = ?
        ORDER BY created_at DESC
        LIMIT 10
    "#,
                PostRow::COLUMNS
            )))
            .bind(id)
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                println!("{:#?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })
        },
        async {
            sqlx::query_as::<_, ProductRow>(AssertSqlSafe(format!(
                r#"SELECT {} FROM products
        WHERE user_id = ?
        ORDER BY created_at DESC
        LIMIT 10
    "#,
                ProductRow::COLUMNS
            )))
            .bind(id)
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                println!("{:#?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })
        }
    )?;

    Ok((
        StatusCode::OK,
        Json(UserSingleResource::from((user, posts, products))),
    ))
}
