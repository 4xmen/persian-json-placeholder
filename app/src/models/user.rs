use macros::resource;
// this is an impl from generated model
use super::User;

impl User {}


resource! {
    pub UserResource from User{
        id,
        name,
        email,
        created_at,
        updated_at,
    }
}