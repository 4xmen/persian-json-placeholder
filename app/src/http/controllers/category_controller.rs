// main model
use crate::models::Category;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use rustavel_core::config::CONFIG;
use rustavel_core::config::database::DbPoolType;
use serde::Serialize;
use crate::models::category::{CategoryResource, SingleCategory};

pub async fn index(State(pool): State<DbPoolType>) -> Result<impl IntoResponse, StatusCode> {
    let sql = r#"
        SELECT * FROM categories
    "#;

    let cats = sqlx::query_as::<_, Category>(sql)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            println!("{:#?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(
       CategoryResource::collection(cats)
    ))
}

pub async fn show(State(pool): State<DbPoolType>, Path(slug): Path<String>) -> impl IntoResponse {
    let cat_sql = "SELECT * FROM categories WHERE slug = ?";

    let category = match sqlx::query_as::<_, Category>(cat_sql)
        .bind(&slug)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(category)) => category,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let parent_sql = "SELECT * FROM categories WHERE id = ?";

    let child_sql = "SELECT * FROM categories WHERE parent_id = ?";

    let parent_id = category.parent_id;

    let (parent, children) = match tokio::try_join!(
        async {
            match parent_id {
                Some(id) => {
                    sqlx::query_as::<_, Category>(parent_sql)
                        .bind(id)
                        .fetch_optional(&pool)
                        .await
                }
                None => Ok(None),
            }
        },
        sqlx::query_as::<_, Category>(child_sql)
            .bind(category.id)
            .fetch_all(&pool),
    ) {
        Ok((parent, children)) => (parent, children),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };


    Json(SingleCategory::from((category,parent,children))).into_response()
}
