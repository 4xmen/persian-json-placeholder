use crate::helper::general::asset_url;
use macros::project;
use macros::resource;
use rustavel_core::config::database::PKeyType;
use rustavel_core::route;
// this is an impl from generated model
use super::Post;
use crate::helper::http_functions::ROUTE_MANAGER;
use crate::models::comment::{CommentResource, CommentRow};
use crate::models::user::UserSimple;
use crate::post_fields;

impl Post {}

post_fields!(PostRow: except(user_id));

resource! {
    pub PostsResource from PostRow{
        id,
        title,
        body,
        image = asset_url(Some(&this.image)),
        created_at,
        updated_at,
        url = route!("api.v1.posts.show",this.slug).unwrap_or_default()
    }
}
resource! {
    pub PostResource from (Post,UserSimple,Vec<CommentRow>){
        id = this.0.id,
        title = this.0.title,
        body= this.0.body,
        user : UserSimpleResource = this.1,
        image = asset_url(Some(&this.0.image)),
        created_at = this.0.created_at,
        updated_at = this.0.updated_at,
        url = route!("api.v1.posts.show",this.0.slug).unwrap_or_default(),
        comments : Vec<CommentResource> = this.2,
    }
}
