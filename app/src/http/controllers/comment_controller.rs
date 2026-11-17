// main model
use crate::models::Comment;

use axum::extract::{RawPathParams, RawQuery, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use rustavel_core::state::AppState;

pub async fn index(State(_state): State<AppState>) -> impl IntoResponse {
    (StatusCode::OK, "Comment index")
}

pub async fn create(State(_state): State<AppState>) -> impl IntoResponse {
    (StatusCode::OK, "to create Comment called")
}

pub async fn store(State(_state): State<AppState>, params: RawPathParams) -> impl IntoResponse {
    (
        StatusCode::OK,
        println!("to store Comment called request: {:?}", params),
    )
}

pub async fn edit(State(_state): State<AppState>, query: RawQuery) -> impl IntoResponse {
    (
        StatusCode::OK,
        println!("to edit Comment called id: {:?}", query),
    )
}

pub async fn update(
    State(_state): State<AppState>,
    query: RawQuery,
    params: RawPathParams,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        println!(
            "to edit Comment called id and request: {:?}, {:?}",
            query, params
        ),
    )
}

pub async fn show(State(_state): State<AppState>, query: RawQuery) -> impl IntoResponse {
    (
        StatusCode::OK,
        println!("to show Comment called id: {:?}", query),
    )
}

pub async fn destroy(State(_state): State<AppState>, query: RawQuery) -> impl IntoResponse {
    (
        StatusCode::OK,
        println!("to destroy Comment called id: {:?}", query),
    )
}
