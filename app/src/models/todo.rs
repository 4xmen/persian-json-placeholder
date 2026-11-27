use super::Todo;
use macros::resource;

impl Todo {}

resource! {
    pub TodoResource from Todo{
        id,
        title,
        done,
        created_at,
    }
}
