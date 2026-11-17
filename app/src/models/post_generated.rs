//!  this code generating by system
//!  if you want edit create backup from your modified code

use rustavel_core::mvc::model::Model;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub slug: String,
    pub body: Option<String>,
    pub user_id: u64,
    pub image: String,
    pub created_at: Option<time::PrimitiveDateTime>,
    pub updated_at: Option<time::PrimitiveDateTime>,
}

impl Model for Post {
    type PrimaryKey = u64;

    fn table() -> &'static str {
        "posts"
    }
    fn primary_key() -> &'static str {
        "id"
    }
    fn columns() -> &'static [&'static str] {
        &["id", "title", "slug", "body", "user_id", "image", "created_at", "updated_at"]
    }
}