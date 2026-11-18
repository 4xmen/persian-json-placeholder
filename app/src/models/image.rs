use crate::helper::general::asset_url;
use serde::{Deserialize, Serialize};
// this is an impl from generated model
use super::Image;

impl Image {}

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct ImageResponse {
    pub image: Option<String>,
    pub thumb: Option<String>,
}

impl From<Image> for ImageResponse {
    fn from(img: Image) -> Self {
        Self {
            image: asset_url(&Some(img.image.as_str().to_owned())),
            thumb: asset_url(&Some(img.thumb.as_str().to_owned())),
        }
    }
}
