// use std::vec;
use crate::models::Todo;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use rustavel_core::config::database::DbPoolType;

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

    Ok(Json(todos))
}
