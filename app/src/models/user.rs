use crate::user_fields;
use macros::{project, resource};
use rustavel_core::config::database::PKeyType;
use rustavel_core::route;
use crate::helper::http_functions::ROUTE_MANAGER;
// this is an impl from generated model
use super::User;

impl User {}

user_fields!(UserList: except(password, remember_token));
user_fields!(UserCombo: only(id,name));

resource! {
    UserListResource from UserList {
        id,
        name,
        email,
        email_verified_at,
        created_at,
        updated_at,
        url = route!("user.show",this.email).unwrap()
    }
}
