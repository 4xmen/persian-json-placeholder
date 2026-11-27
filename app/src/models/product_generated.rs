//!  this code generating by system
//!  if you want edit create backup from your modified code

use rustavel_core::mvc::model::Model;
use serde::{Deserialize, Serialize};
use rustavel_core::config::database::PKeyType;

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Product {
    pub id: PKeyType,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub price: u64,
    pub quantity: i32,
    pub category_id: Option<u64>,
    pub user_id: u64,
    pub image: Option<String>,
    pub thumb: Option<String>,
    pub created_at: Option<time::PrimitiveDateTime>,
    pub updated_at: Option<time::PrimitiveDateTime>,
}

impl Model for Product {
    type PrimaryKey = PKeyType;

    fn table() -> &'static str {
        "products"
    }
    fn primary_key() -> &'static str {
        "id"
    }
    fn columns() -> &'static [&'static str] {
        &["id", "name", "description", "slug", "price", "quantity", "category_id", "user_id", "image", "thumb", "created_at", "updated_at"]
    }
}