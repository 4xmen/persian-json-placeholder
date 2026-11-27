use rustavel_core::config::database::PKeyType;
use macros::project;
use macros::resource;
use crate::comment_fields;
// this is an impl from generated model
use super::Comment;

impl Comment {}

comment_fields!(CommentRow: except(commentable_id,commentable_type,updated_at));
resource! {
    pub CommentResource from CommentRow{
        id,
        email,
        author,
        body,
        created_at,
    }
}