// use std::vec;
use crate::models::Todo;
use crate::models::todo::TodoResource;
use axum::Json;
use axum::extract::{RawPathParams, RawQuery, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use rustavel_core::config::database::DbPoolType;
use rustavel_core::helper::datetime::TimeExt;
use rustavel_core::state::AppState;
use time::macros::datetime;

pub async fn index(State(pool): State<DbPoolType>) -> Result<impl IntoResponse, StatusCode> {
    let sql = r#"
        SELECT * FROM todos
        WHERE deleted_at IS NULL
    "#;

    let todos = sqlx::query_as::<_, Todo>(sql)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            println!("{:#?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(TodoResource::collection(todos)))
}
pub async fn create(State(_state): State<AppState>) -> impl IntoResponse {
    // Todo::all();
    println!("so so...");
    (StatusCode::OK, "to create called")
}
pub async fn store(State(_state): State<AppState>) -> impl IntoResponse {
    (StatusCode::OK, "to store called")
}
pub async fn edit(State(_state): State<AppState>, params: RawPathParams) -> impl IntoResponse {
    (StatusCode::OK, println!("to edit called id: {:?}", params))
}
#[axum::debug_handler]
pub async fn update(State(_state): State<AppState>, params: RawPathParams) -> impl IntoResponse {
    (StatusCode::OK, println!("to edit called id: {:?}", params))
}
pub async fn show(State(_state): State<AppState>, params: RawPathParams) -> impl IntoResponse {
    let x = datetime!(2026-04-12 15:20:20);

    println!("{},{}", x.diff_for_humans(), x.ldate("Y/m/d H:i:s F l"));
    (
        StatusCode::OK,
        format!(
            "to edit called id: {:?},{}",
            params,
            x.format_php("Y/m/d H:i:s w")
        ),
    )
}

pub async fn destroy(
    State(_state): State<AppState>,
    params: RawPathParams,
    query: RawQuery,
) -> impl IntoResponse {
    // for (key, value) in &params {
    //     println!("{key:?} = {value:?}");
    // }
    (
        StatusCode::OK,
        println!(
            "to destroy called id:  {:?}, {:?}, {:?}",
            _state, query, params
        ),
    )
}
