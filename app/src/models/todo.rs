use super::Todo;
use crate::helper::http_functions::ROUTE_MANAGER;
use macros::resource;
use rustavel_core::config::database::PKeyType;
use rustavel_core::route;

impl Todo {}

resource! {
    pub TodoResource from Todo{
        id,
        title,
        done,
        created_at,
        url = resource_url(this.id)
    }
}




fn resource_url(id: PKeyType) -> String {
    route!("todos.show", id).unwrap_or_default()
}
