use crate::helper::general::asset_url;
use crate::product_fields;
use macros::{project, resource};
use rustavel_core::config::database::PKeyType;
use rustavel_core::route;
// this is an impl from generated model
use super::{Category, Image, Product};
use crate::helper::http_functions::ROUTE_MANAGER;
use crate::models::category::CategoryResource;
use crate::models::comment::{CommentResource, CommentRow};
use crate::models::image::ImageResource;
use crate::models::user::UserSimple;

impl Product {}

product_fields!(ProductRow: except(user_id,category_id));

resource! {
    pub ProductRowResource from ProductRow {
        id,
        name,
        description,
        price,
        quantity,
        thumb = asset_url(this.thumb.as_deref()),
        image = asset_url(this.image.as_deref()),
        url = route!("api.v1.products.show", this.slug).unwrap_or_default(),
        created_at,
        updated_at,
    }
}

resource! {
    pub ProductSingleResource from (Product, UserSimple,Vec<Category>,Vec<Image>, Vec<CommentRow>) {
        id = this.0.id,
        name = this.0.name,
        description = this.0.description,
        price = this.0.price,
        quantity = this.0.quantity,
        thumb = asset_url(this.0.thumb.as_deref()),
        image = asset_url(this.0.image.as_deref()),
        created_at = this.0.created_at,
        updated_at = this.0.updated_at,
        user: UserSimpleResource = this.1,
        categories: Vec<CategoryResource> = this.2,
        images: Vec<ImageResource> = this.3,
        comments: Vec<CommentResource> = this.4,
    }
}
