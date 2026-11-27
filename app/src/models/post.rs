use crate::helper::general::asset_url;
use crate::models::user::UserResource;
use macros::resource;
use serde::Serialize;
use rustavel_core::route;
// this is an impl from generated model
use super::{Post, User};
use crate::helper::http_functions::ROUTE_MANAGER;

impl Post {}

#[derive(Serialize)]
pub struct PostResponse {
    pub id: u64,
    pub title: String,
    pub slug: String,
    pub body: Option<String>,
    pub user_id: u64,
    pub image: Option<String>,
    pub created_at: Option<time::PrimitiveDateTime>,
    pub updated_at: Option<time::PrimitiveDateTime>,
    pub url: String,
}

resource! {
    pub PostsResource from Post{
        id,
        title,
        body,
        image = asset_url(Some(&this.image)),
        created_at = this.created_at,
        updated_at = this.updated_at,
        url = route!("post.show",this.slug).unwrap_or_default()
    }
}
resource! {
    pub PostResource from (Post,User){
        id = this.0.id,
        title = this.0.title,
        body= this.0.body,
        user : UserResource = this.0,
        image = asset_url(Some(&this.0.image)),
        created_at = this.0.created_at,
        updated_at = this.0.updated_at,
        url = route!("post.show",this.0.slug).unwrap_or_default()
    }
}


