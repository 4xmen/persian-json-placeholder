// main model
use crate::models::{Category, Comment, Image, Post, Product};

use axum::extract::{OriginalUri, Path, Query, RawPathParams, RawQuery, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use rustavel_core::config::CONFIG;
use rustavel_core::config::database::DbPoolType;
use rustavel_core::state::AppState;
use crate::helper::general::{asset_url, PaginateBuilder};
use crate::http::controllers::post_controller::{PaginationQuery, UserResponse};
use crate::models::category::CategoryResponse;
use crate::models::image::ImageResponse;
use crate::models::post::PostResponse;
use crate::models::product::ProductResponse;



#[derive(Serialize)]
pub struct SingleProductResponse {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub price: u64,
    pub quantity: i32,
    pub image: Option<String>,
    pub thumb: Option<String>,
    pub created_at: Option<time::OffsetDateTime>,
    pub updated_at: Option<time::OffsetDateTime>,

    pub url: String,

    pub user: UserResponse,
    pub categories: Vec<CategoryResponse>,
    pub images: Vec<ImageResponse>,
}

#[derive(Serialize)]
pub struct ProductShowResponse {
    pub product: SingleProductResponse,
    pub comments: Vec<Comment>,
}

pub async fn index(
    State(pool): State<DbPoolType>,
    OriginalUri(uri): OriginalUri,
    Query(query): Query<PaginationQuery>,
) -> impl IntoResponse {

    let page = query.page.unwrap_or(1).max(1);
    let max = query.max.unwrap_or(10).clamp(1, 100);

    // Total count
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM products"
    )
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    let count = count as usize;

    // Pagination offset
    let offset = (page - 1) * max;

    // Fetch products
    let sql = r#"
        SELECT *
        FROM products
        ORDER BY id DESC
        LIMIT ? OFFSET ?
    "#;

    let products: Vec<Product> = sqlx::query_as(sql)
        .bind(max as i64)
        .bind(offset as i64)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    // Convert Product -> ProductResponse
    let data: Vec<ProductResponse> = products
        .into_iter()
        .map(ProductResponse::from)
        .collect();

    // Build pagination response
    let pagination = PaginateBuilder::new(
        max,
        count,
        page,
        uri.to_string(),
    ).build(data);

    Json(pagination)
}

pub async fn show(
    State(pool): State<DbPoolType>,
    Path(slug): Path<String>,
) -> impl IntoResponse {

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
        SELECT *
        FROM comments
        WHERE commentable_type = 'Model::Product'
        AND commentable_id = ?
    "#;

    let sql_images = r#"
        SELECT id, product_id, image, thumb
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
        sqlx::query_as::<_, UserResponse>(sql_user)
            .bind(product.user_id)
            .fetch_one(&pool),

        sqlx::query_as::<_, Comment>(sql_comments)
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
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let response = SingleProductResponse {
        id: product.id,
        name: product.name,
        description: product.description,
        slug: product.slug.clone(),
        price: product.price,
        quantity: product.quantity,
        image: asset_url(&product.image),
        thumb: asset_url(&product.thumb),
        created_at: product.created_at,
        updated_at: product.updated_at,
        url: format!("{}/api/v1/products",CONFIG.app.url),

        user,
        categories: categories.into_iter()
            .map(|cat| CategoryResponse::from((cat, format!("{}/api/v1/categories", CONFIG.app.url))))
            .collect(),
        images: images.into_iter().map(ImageResponse::from).collect(),
    };

    Json(ProductShowResponse {
        product: response,
        comments,
    }).into_response()
}

// pub async fn create(State(_state): State<AppState>) -> impl IntoResponse {
//     (StatusCode::OK, "to create Product called")
// }
// 
// pub async fn store(State(_state): State<AppState>, params: RawPathParams) -> impl IntoResponse {
//     (
//         StatusCode::OK,
//         println!("to store Product called request: {:?}", params),
//     )
// }
// 
// pub async fn edit(State(_state): State<AppState>, query: RawQuery) -> impl IntoResponse {
//     (
//         StatusCode::OK,
//         println!("to edit Product called id: {:?}", query),
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
//             "to edit Product called id and request: {:?}, {:?}",
//             query, params
//         ),
//     )
// }

// pub async fn show(
//     State(pool): State<DbPoolType>,
//     Path(slug): Path<String>,
// ) -> impl IntoResponse {
//
//     let sql_post = "SELECT * FROM products WHERE slug = ?";
//     let sql_user = "SELECT id, name, email FROM users WHERE id = ?";
//
//     let sql_comment = r#"
//         SELECT *
//         FROM comments
//         WHERE commentable_type = 'Model::Product'
//         AND commentable_id = ?
//     "#;
//
//     let sql_extra_images = "SELECT * FROM products WHERE product_id = ?";
//     let sql_categories_list = r#"SELECT category_id FROM product_category
//     WHERE product_id = ?"#;
//     let sql_categories = "SELECT * FROM categories WHERE id IN(?)";
// }

// pub async fn destroy(State(_state): State<AppState>, query: RawQuery) -> impl IntoResponse {
//     (
//         StatusCode::OK,
//         println!("to destroy Product called id: {:?}", query),
//     )
// }
