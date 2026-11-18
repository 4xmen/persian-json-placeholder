//!  this code generating by system
//!  if you want edit create backup from your modified code

use rustavel_core::mvc::model::Model;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Comment {
    pub id: u64,
    pub body: String,
    pub author: String,
    pub email: String,
    pub commentable_type: String,
    pub commentable_id: u64,
    pub created_at: Option<time::OffsetDateTime>,
    pub updated_at: Option<time::OffsetDateTime>,
}

impl Model for Comment {
    type PrimaryKey = u64;

    fn table() -> &'static str {
        "comments"
    }
    fn primary_key() -> &'static str {
        "id"
    }
    fn columns() -> &'static [&'static str] {
        &[
            "id",
            "body",
            "author",
            "email",
            "commentable_type",
            "commentable_id",
            "created_at",
            "updated_at",
        ]
    }
}
