
// main model
use crate::models::{Post, Product, User};

use axum::extract::{Path, RawPathParams, RawQuery, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use sqlx::FromRow;
use rustavel_core::config::CONFIG;
use rustavel_core::config::database::DbPoolType;
use rustavel_core::state::AppState;

#[derive(Serialize, FromRow)]
pub struct UserResponseWithoutRelations  {
    id: u64,
    name: String,
    email: String,
    created_at: Option<time::OffsetDateTime>,
    updated_at: Option<time::OffsetDateTime>,
}
#[derive(Serialize, FromRow)]
pub struct UserResponse {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub posts: Vec<Post>,
    pub products: Vec<Product>,
    pub created_at: Option<time::OffsetDateTime>,
    pub updated_at: Option<time::OffsetDateTime>,
}
pub async fn index(
    State(pool): State<DbPoolType>,
) -> Result<impl IntoResponse, StatusCode> {
    let sql = r#"
        SELECT id, name, email,
        created_at,
        updated_at
        FROM users
        WHERE deleted_at IS NULL
    "#;

    let users = sqlx::query_as::<_, UserResponseWithoutRelations>(sql)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            println!("{:#?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(users))
}


pub async fn create(State(_state): State<AppState>) -> impl IntoResponse {
    (StatusCode::OK, "to create User called")
}

pub async fn store(State(_state): State<AppState>, params: RawPathParams) -> impl IntoResponse {
    (
        StatusCode::OK,
        println!("to store User called request: {:?}", params),
    )
}

pub async fn edit(State(_state): State<AppState>, query: RawQuery) -> impl IntoResponse {
    (StatusCode::OK, println!("to edit User called id: {:?}", query))
}

pub async fn update(
    State(_state): State<AppState>,
    query: RawQuery,
    params: RawPathParams,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        println!("to edit User called id and request: {:?}, {:?}", query, params),
    )
}

pub async fn show(
    State(pool): State<DbPoolType>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, StatusCode> {

    let user_sql = r#"
        SELECT id, name, email, created_at, updated_at
        FROM users
        WHERE deleted_at IS NULL AND id = ?
    "#;

    let posts_sql = r#"
        SELECT *
        FROM posts
        WHERE user_id = ?
        ORDER BY created_at DESC
        LIMIT 10
    "#;

    let products_sql = r#"
        SELECT *
        FROM products
        WHERE user_id = ?
        ORDER BY created_at DESC
        LIMIT 10
    "#;

    let (user, posts, products) = tokio::try_join!(
        async {
            sqlx::query_as::<_, UserResponseWithoutRelations>(user_sql)
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
            sqlx::query_as::<_, Post>(posts_sql)
                .bind(id)
                .fetch_all(&pool)
                .await
                .map_err(|e| {
                    println!("{:#?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })
        },
        async {
            sqlx::query_as::<_, Product>(products_sql)
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
        Json(UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
            posts,
            products,
        }),
    ))
}

pub async fn destroy(State(_state): State<AppState>, query: RawQuery) -> impl IntoResponse {
    (
        StatusCode::OK,
        println!("to destroy User called id: {:?}", query),
    )
}

