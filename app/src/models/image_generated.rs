//!  this code generating by system
//!  if you want edit create backup from your modified code

use rustavel_core::config::database::PKeyType;
use rustavel_core::mvc::model::Model;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Image {
    pub id: PKeyType,
    pub product_id: u64,
    pub image: String,
    pub thumb: String,
}

impl Model for Image {
    type PrimaryKey = PKeyType;

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

/// the macro that the generator places next to each model generate projection of Image
#[macro_export]
macro_rules! image_fields {
    ($($rest:tt)*) => {
        project! {
            Image {
                id: PKeyType,
                product_id: u64,
                image: String,
                thumb: String,
            }
            $($rest)*
        }
    };
}
