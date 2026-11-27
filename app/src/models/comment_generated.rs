//!  this code generating by system
//!  if you want edit create backup from your modified code

use rustavel_core::mvc::model::Model;
use serde::{Deserialize, Serialize};
use rustavel_core::config::database::PKeyType;

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Comment {
    pub id: PKeyType,
    pub body: String,
    pub author: String,
    pub email: String,
    pub commentable_type: String,
    pub commentable_id: PKeyType,
    pub created_at: Option<time::PrimitiveDateTime>,
    pub updated_at: Option<time::PrimitiveDateTime>,
}

impl Model for Comment {
    type PrimaryKey = PKeyType;

    fn table() -> &'static str {
        "comments"
    }
    fn primary_key() -> &'static str {
        "id"
    }
    fn columns() -> &'static [&'static str] {
        &["id", "body", "author", "email", "commentable_type", "commentable_id", "created_at", "updated_at"]
    }
}