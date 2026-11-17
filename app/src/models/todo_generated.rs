//!  this code generating by system
//!  if you want edit create backup from your modified code

use rustavel_core::mvc::model::Model;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub done: bool,
    pub created_at: Option<time::PrimitiveDateTime>,
    pub updated_at: Option<time::PrimitiveDateTime>,
    pub deleted_at: Option<time::PrimitiveDateTime>,
}

impl Model for Todo {
    type PrimaryKey = u64;

    fn table() -> &'static str {
        "todos"
    }
    fn primary_key() -> &'static str {
        "id"
    }
    fn columns() -> &'static [&'static str] {
        &[
            "id",
            "title",
            "done",
            "created_at",
            "updated_at",
            "deleted_at",
        ]
    }
}
