use super::Todo;
use macros::resource;
use rustavel_core::route;
use crate::helper::http_functions::ROUTE_MANAGER;

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

fn resource_url(id: i64) -> String {
    route!("todos.show", id).unwrap_or_default()
}


