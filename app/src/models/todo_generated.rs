//!  this code generating by system
//!  if you want edit create backup from your modified code

use rustavel_core::config::database::PKeyType;
use rustavel_core::mvc::model::Model;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Todo {
    pub id: PKeyType,
    pub title: String,
    pub done: bool,
    pub created_at: Option<time::PrimitiveDateTime>,
    pub updated_at: Option<time::PrimitiveDateTime>,
    pub deleted_at: Option<time::PrimitiveDateTime>,
}

impl Model for Todo {
    type PrimaryKey = PKeyType;

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

/// the macro that the generator places next to each model generate projection of Todo
#[macro_export]
macro_rules! todo_fields {
    ($($rest:tt)*) => {
        project! {
            Todo {
                id: PKeyType,
                title: String,
                done: bool,
                created_at: Option<time::PrimitiveDateTime>,
                updated_at: Option<time::PrimitiveDateTime>,
                deleted_at: Option<time::PrimitiveDateTime>,
            }
            $($rest)*
        }
    };
}
