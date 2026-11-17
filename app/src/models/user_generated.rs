//!  this code generating by system
//!  if you want edit create backup from your modified code

use rustavel_core::mvc::model::Model;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub email_verified_at: Option<time::PrimitiveDateTime>,
    pub password: String,
    pub remember_token: Option<String>,
    pub created_at: Option<time::PrimitiveDateTime>,
    pub updated_at: Option<time::PrimitiveDateTime>,
    pub deleted_at: Option<time::PrimitiveDateTime>,
}

impl Model for User {
    type PrimaryKey = u64;

    fn table() -> &'static str {
        "users"
    }
    fn primary_key() -> &'static str {
        "id"
    }
    fn columns() -> &'static [&'static str] {
        &[
            "id",
            "name",
            "email",
            "email_verified_at",
            "password",
            "remember_token",
            "created_at",
            "updated_at",
            "deleted_at",
        ]
    }
}
