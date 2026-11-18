use serde::{Deserialize, Serialize};
use crate::helper::general::asset_url;
// this is an impl from generated model
use super::{Category};

impl Category {}


#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CategoryResponse {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub parent_id: Option<u64>,
    pub image: Option<String>,
    pub thumb: Option<String>,
    pub url: String,
}

impl From<(Category, String)> for CategoryResponse {
    fn from((category, base_url): (Category, String)) -> Self {
        Self{
            id: category.id,
            name: category.name,
            description: category.description,
            slug: category.slug.clone(),
            parent_id: category.parent_id,
            image: asset_url(&category.image),
            thumb: asset_url(&category.thumb),
            url: format!("{}/{}", base_url, category.slug),
        }
    }
}