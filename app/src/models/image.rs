use crate::helper::general::asset_url;
use macros::resource;
// this is an impl from generated model
use super::Image;

impl Image {}

resource! {
    pub ImageResource from Image {
        id,
        image = asset_url(Some(&this.image)),
        thumb = asset_url(Some(&this.image)),
    }
}
