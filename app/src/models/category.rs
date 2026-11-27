use axum_extra::routing::Resource;
use crate::helper::general::asset_url;
use macros::resource;
use rustavel_core::route;
// this is an impl from generated model
use super::Category;
use crate::helper::http_functions::ROUTE_MANAGER;

impl Category {}

resource! {
    pub CategoryResource from Category {
        id,
        name,
        description,
        thumb = asset_url(this.thumb.as_deref()),
        image = asset_url(this.image.as_deref()),
        url = route!("api.v1.categories.show", this.slug).unwrap_or_default(),

    }
}

resource! {
    pub SingleCategory from (Category,Option<Category>,Vec<Category>) {
        id = this.0.id,
        name = this.0.name,
        description = this.0.description,
        thumb = asset_url(this.0.thumb.as_deref()),
        image = asset_url(this.0.image.as_deref()),
        url = route!("api.v1.categories.show", this.0.slug).unwrap_or_default(),
        parent : Option<CategoryResource> = this.1,
        children: Vec<CategoryResource> = this.2,
    }
}
