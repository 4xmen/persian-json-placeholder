use crate::helper::general::asset_url;
use serde::Serialize;
// this is an impl from generated model
use super::Product;

impl Product {}

#[derive(Serialize)]
pub struct ProductResponse {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub price: u64,
    pub quantity: i32,
    pub category_id: Option<u64>,
    pub user_id: u64,
    pub image: Option<String>,
    pub thumb: Option<String>,
    pub created_at: Option<time::OffsetDateTime>,
}

impl From<Product> for ProductResponse {
    fn from(product: Product) -> Self {
        Self {
            id: product.id,
            name: product.name,
            description: product.description,
            slug: product.slug,
            price: product.price,
            quantity: product.quantity,
            category_id: product.category_id,
            user_id: product.user_id,
            image: asset_url(&product.image),
            thumb: asset_url(&product.thumb),
            created_at: product.created_at,
        }
    }
}
