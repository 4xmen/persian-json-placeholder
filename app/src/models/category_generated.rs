//!  this code generating by system
//!  if you want edit create backup from your modified code

use rustavel_core::mvc::model::Model;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Category {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub parent_id: Option<u64>,
    pub image: Option<String>,
    pub thumb: Option<String>,
}

impl Model for Category {
    type PrimaryKey = u64;

    fn table() -> &'static str {
        "categories"
    }
    fn primary_key() -> &'static str {
        "id"
    }
    fn columns() -> &'static [&'static str] {
        &["id", "name", "description", "slug", "parent_id", "image", "thumb"]
    }
}