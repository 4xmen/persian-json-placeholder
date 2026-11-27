//!  this code generating by system
//!  if you want edit create backup from your modified code

use rustavel_core::config::database::PKeyType;
use rustavel_core::mvc::model::Model;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct User {
    pub id: PKeyType,
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
    type PrimaryKey = PKeyType;

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

/// the macro that the generator places next to each model generate projection of User
#[macro_export]
macro_rules! user_fields {
    ($($rest:tt)*) => {
        project! {
            User {
                id: PKeyType,
                name: String,
                email: String,
                email_verified_at: Option<time::PrimitiveDateTime>,
                password: String,
                remember_token: Option<String>,
                created_at: Option<time::PrimitiveDateTime>,
                updated_at: Option<time::PrimitiveDateTime>,
                deleted_at: Option<time::PrimitiveDateTime>,
            }
            $($rest)*
        }
    };
}
