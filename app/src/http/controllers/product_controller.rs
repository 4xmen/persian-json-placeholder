// main model
use crate::models::Product;

use axum::extract::{RawPathParams, RawQuery, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use rustavel_core::state::AppState;

pub async fn index(State(_state): State<AppState>) -> impl IntoResponse {
    (StatusCode::OK, "Product index")
}

pub async fn create(State(_state): State<AppState>) -> impl IntoResponse {
    (StatusCode::OK, "to create Product called")
}

pub async fn store(State(_state): State<AppState>, params: RawPathParams) -> impl IntoResponse {
    (
        StatusCode::OK,
        println!("to store Product called request: {:?}", params),
    )
}

pub async fn edit(State(_state): State<AppState>, query: RawQuery) -> impl IntoResponse {
    (
        StatusCode::OK,
        println!("to edit Product called id: {:?}", query),
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
            "to edit Product called id and request: {:?}, {:?}",
            query, params
        ),
    )
}

pub async fn show(State(_state): State<AppState>, query: RawQuery) -> impl IntoResponse {
    (
        StatusCode::OK,
        println!("to show Product called id: {:?}", query),
    )
}

pub async fn destroy(State(_state): State<AppState>, query: RawQuery) -> impl IntoResponse {
    (
        StatusCode::OK,
        println!("to destroy Product called id: {:?}", query),
    )
}
