use serde::Serialize;
use crate::helper::general::asset_url;
use crate::http::controllers::user_controller::UserResponse;
// this is an impl from generated model
use super::{Post};

impl Post {}


#[derive(Serialize)]
pub struct PostResponse {
    pub id: u64,
    pub title: String,
    pub slug: String,
    pub body: Option<String>,
    pub user_id: u64,
    pub image: Option<String>,
    pub created_at: Option<time::OffsetDateTime>,
    pub updated_at: Option<time::OffsetDateTime>,
    pub url: String,
}


impl From<(Post, String)> for PostResponse {
    fn from((post, base_url): (Post, String)) -> Self {
        Self {
            id: post.id,
            title: post.title,
            body: post.body,
            slug: post.slug.clone(),
            user_id: post.user_id,
            image: asset_url(&Some(post.image.as_str().to_owned())),
            created_at: post.created_at,
            updated_at: post.updated_at,
            url: format!("{}/{}", base_url,post.slug),
        }
    }
}