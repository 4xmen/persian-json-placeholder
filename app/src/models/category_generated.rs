//!  this code generating by system
//!  if you want edit create backup from your modified code

use rustavel_core::mvc::model::Model;
use serde::{Deserialize, Serialize};
use rustavel_core::config::database::PKeyType;

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Category {
    pub id: PKeyType,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub parent_id: Option<u64>,
    pub image: Option<String>,
    pub thumb: Option<String>,
}

impl Model for Category {
    type PrimaryKey = PKeyType;

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

/// the macro that the generator places next to each model generate projection of Category
#[macro_export]
macro_rules! category_fields {
    ($($rest:tt)*) => {
        project! {
            Category {
                id: PKeyType,
                name: String,
                description: Option<String>,
                slug: String,
                parent_id: Option<u64>,
                image: Option<String>,
                thumb: Option<String>,
            }
            $($rest)*
        }
    };
}