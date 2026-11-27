use crate::user_fields;
use macros::{project, resource};
use rustavel_core::config::database::PKeyType;
use rustavel_core::route;
use crate::helper::http_functions::ROUTE_MANAGER;
use crate::models::post::{PostRow, PostsResource};
use crate::models::product::{ProductRow, ProductRowResource};
// this is an impl from generated model
use super::User;

impl User {}

user_fields!(UserList: except(password, remember_token, email_verified_at));
user_fields!(UserSimple: only(id, name, email));

resource! {
    pub UserListResource from UserList {
        id,
        name,
        email,
        created_at,
        updated_at,
        url = route!("api.v1.users.show",this.id).unwrap()
    }
}

resource! {
    pub UserSimpleResource from UserSimple {
        id,
        name,
        email,
        url = route!("api.v1.users.show",this.id).unwrap()
    }
}

resource! {
    pub UserSingleResource from (UserList, Vec<PostRow>,Vec<ProductRow>){
        id = this.0.id,
        name = this.0.name,
        email = this.0.email,
        created_at = this.0.created_at,
        updated_at = this.0.updated_at,
        posts : Vec<PostsResource> = this.1,
        products : Vec<ProductRowResource> = this.2,
    }
}
