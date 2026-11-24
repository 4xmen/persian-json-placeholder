use minijinja::value::Rest;
use minijinja::{Environment, Error, ErrorKind, Value};
use rustavel_core::route::route_manager::RouteManager;
use std::sync::{Arc, OnceLock};

pub static ROUTE_MANAGER: OnceLock<Arc<RouteManager>> = OnceLock::new();

pub fn register_route_function(env: &mut Environment) {
    env.add_function("route", move |name: String, params: Rest<Value>| {
        let values: Vec<String> = params.iter().map(ToString::to_string).collect();
        let display_params: Vec<&dyn std::fmt::Display> = values
            .iter()
            .map(|value| value as &dyn std::fmt::Display)
            .collect();

        ROUTE_MANAGER
            .get()
            .expect("RouteManager not initialized")
            .try_url(&name, &display_params)
            .map_err(|err| Error::new(ErrorKind::InvalidOperation, err.to_string()))
    });
}
