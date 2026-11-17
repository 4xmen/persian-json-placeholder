//!  this code generating by system
//!  if you want edit create backup from your modified code

use rustavel_core::mvc::model::Model;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Image {
    pub id: u64,
    pub product_id: u64,
    pub image: String,
    pub thumb: String,
}

impl Model for Image {
    type PrimaryKey = u64;

    fn table() -> &'static str {
        "images"
    }
    fn primary_key() -> &'static str {
        "id"
    }
    fn columns() -> &'static [&'static str] {
        &["id", "product_id", "image", "thumb"]
    }
}