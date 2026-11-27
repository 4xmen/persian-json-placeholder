// main model
use crate::models::{Category, Comment, Image, Product};

use crate::helper::general::{ asset_url};
use crate::http::controllers::post_controller::{PaginationQuery};
use axum::Json;
use axum::extract::{OriginalUri, Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::AssertSqlSafe;
use rustavel_core::config::database::DbPoolType;
use crate::helper::paginator::PaginateBuilder;
use crate::models::comment::CommentRow;
use crate::models::product::{ProductRow, ProductRowResource, ProductSingleResource};
use crate::models::user::UserSimple;

pub async fn index(
    State(pool): State<DbPoolType>,
    OriginalUri(uri): OriginalUri,
    Query(query): Query<PaginationQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(1).max(1);
    let max = query
        .max
        .or(query.count)
        .unwrap_or(10)
        .clamp(1, 100);


    // Total count
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM products")
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    let count = count as usize;

    // Pagination offset
    let offset = (page - 1) * max;

    // Fetch products

    let products: Vec<ProductRow> = sqlx::query_as(AssertSqlSafe(format!(r#"
        SELECT {} FROM products
        ORDER BY id DESC
        LIMIT ? OFFSET ?
    "#, ProductRow::COLUMNS)))
        .bind(max as i64)
        .bind(offset as i64)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    // Convert Product -> ProductResponse
    let data = products
        .into_iter()
        .map(ProductRowResource::from)
        .collect::<Vec<_>>();
     // Build pagination response
    let pagination = PaginateBuilder::new(max, count, page, uri.to_string()).build(data);

    Json(pagination)
}

pub async fn show(State(pool): State<DbPoolType>, Path(slug): Path<String>) -> impl IntoResponse {
    let sql_product = "SELECT * FROM products WHERE slug = ?";

    let product = match sqlx::query_as::<_, Product>(sql_product)
        .bind(&slug)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(product)) => product,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let sql_user = "SELECT id, name, email FROM users WHERE id = ?";

    let sql_comments = r#"
        SELECT id, email, author, body, created_at
        FROM comments
        WHERE commentable_type = 'Model::Product'
        AND commentable_id = ?
    "#;

    let sql_images = r#"
        SELECT *
        FROM images
        WHERE product_id = ?
    "#;

    let sql_categories = r#"
        SELECT c.id, c.name, c.description, c.slug,
               c.parent_id, c.image, c.thumb
        FROM categories c
        INNER JOIN product_category pc
            ON pc.category_id = c.id
        WHERE pc.product_id = ?
    "#;

    let (user, comments, images, categories) = match tokio::try_join!(
        sqlx::query_as::<_, UserSimple>(sql_user)
            .bind(product.user_id)
            .fetch_one(&pool),
        sqlx::query_as::<_, CommentRow>(sql_comments)
            .bind(product.id)
            .fetch_all(&pool),
        sqlx::query_as::<_, Image>(sql_images)
            .bind(product.id)
            .fetch_all(&pool),
        sqlx::query_as::<_, Category>(sql_categories)
            .bind(product.id)
            .fetch_all(&pool),
    ) {
        Ok(result) => result,
        Err(db_err) => {
            println!("{:?}",db_err);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response()
        },
    };

    Json( ProductSingleResource::from((product,user,categories,images,comments))).into_response()
}
