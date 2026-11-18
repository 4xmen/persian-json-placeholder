// main model
use crate::models::{Category, Todo};

use crate::models::category::CategoryResponse;
use axum::Json;
use axum::extract::{Path, RawPathParams, RawQuery, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;
use rustavel_core::config::CONFIG;
use rustavel_core::config::database::DbPoolType;
use rustavel_core::state::AppState;

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
        cats.into_iter()
            .map(|post| {
                CategoryResponse::from((post, format!("{}/api/v1/categories", CONFIG.app.url)))
            })
            .collect::<Vec<_>>(),
    ))
}
//
// pub async fn create(State(_state): State<AppState>) -> impl IntoResponse {
//     (StatusCode::OK, "to create Category called")
// }
//
// pub async fn store(State(_state): State<AppState>, params: RawPathParams) -> impl IntoResponse {
//     (
//         StatusCode::OK,
//         println!("to store Category called request: {:?}", params),
//     )
// }
//
// pub async fn edit(State(_state): State<AppState>, query: RawQuery) -> impl IntoResponse {
//     (
//         StatusCode::OK,
//         println!("to edit Category called id: {:?}", query),
//     )
// }
//
// pub async fn update(
//     State(_state): State<AppState>,
//     query: RawQuery,
//     params: RawPathParams,
// ) -> impl IntoResponse {
//     (
//         StatusCode::OK,
//         println!(
//             "to edit Category called id and request: {:?}, {:?}",
//             query, params
//         ),
//     )
// }
//


#[derive(Serialize)]
pub struct CategoryResponseSingle {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub parent_id: Option<u64>,
    pub image: Option<String>,
    pub thumb: Option<String>,

    pub parent: Option<CategoryResponse>,
    pub children: Vec<CategoryResponse>,
}

pub async fn show(
    State(pool): State<DbPoolType>,
    Path(slug): Path<String>,
) -> impl IntoResponse {

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

    let response = CategoryResponseSingle {
        id: category.id,
        name: category.name,
        description: category.description,
        slug: category.slug,
        parent_id: category.parent_id,
        image: category.image,
        thumb: category.thumb,

        parent: parent.map(|cat| CategoryResponse::from((cat, format!("{}/api/v1/categories", CONFIG.app.url)))),

        children: children
            .into_iter()
            .map(|cat| CategoryResponse::from((cat, format!("{}/api/v1/categories", CONFIG.app.url)))).collect(),
    };

    Json(response).into_response()
}
//
// pub async fn destroy(State(_state): State<AppState>, query: RawQuery) -> impl IntoResponse {
//     (
//         StatusCode::OK,
//         println!("to destroy Category called id: {:?}", query),
//     )
// }
