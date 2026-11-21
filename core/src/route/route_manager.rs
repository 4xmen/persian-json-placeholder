// src/routes.rs
use std::{
    collections::HashMap,
    fmt::{self, Display, Write},
};

use crate::config::CONFIG;
use anyhow::{Result, anyhow};
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};

/// Characters that stay unescaped in a path segment (RFC 3986 unreserved).
/// Everything else, including `/`, is percent-encoded.
const SEGMENT: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

/// A route pattern split at its placeholders.
///
/// `/todos/show/{user}/{id}` becomes `["/todos/show/", "/", ""]`,
/// so building a URL never has to scan the pattern again.
#[derive(Debug, Clone)]
struct Route {
    parts: Vec<String>,
    literal_len: usize,
}

impl Route {
    /// Splits a pattern into literal parts. Panics on an unclosed `{`,
    /// because the map is generated at startup and this is a programmer error.
    fn parse(pattern: &str) -> Self {
        let mut pieces = pattern.split('{');

        // The text before the first '{' is always plain
        let mut parts = vec![pieces.next().unwrap_or_default().to_string()];

        // Every later piece looks like "name}rest"
        for piece in pieces {
            let (_, rest) = piece
                .split_once('}')
                .expect("unclosed '{' in route pattern");
            parts.push(rest.to_string());
        }

        let literal_len = parts.iter().map(String::len).sum();
        Self { parts, literal_len }
    }

    /// Number of `{param}` placeholders (one fewer than the literal parts).
    fn param_count(&self) -> usize {
        self.parts.len() - 1
    }
}

/// Internal failure reason, shared by the safe and the fallback API.
enum RouteError {
    InvalidName,
    InvalidParams { expected: usize },
}

impl RouteError {
    /// The URL fragment returned by the non-failing `url` methods.
    fn fragment(&self) -> &'static str {
        match self {
            Self::InvalidName => "#invalid-route-name",
            Self::InvalidParams { .. } => "#invalid-parameters",
        }
    }
}

/// Writes text into a `String` while percent-encoding it on the fly,
/// so no intermediate `String` is needed per parameter.
struct Encoder<'a>(&'a mut String);

impl Write for Encoder<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.extend(utf8_percent_encode(s, SEGMENT));
        Ok(())
    }
}

/// Resolves named routes to URLs.
#[derive(Debug, Clone)]
pub struct RouteManager {
    routes: HashMap<String, Route>,
}

impl RouteManager {
    /// Builds the manager from a `name -> pattern` map and precomputes
    /// the literal parts and placeholder count of every route.
    pub fn new(routes: HashMap<String, String>) -> Self {
        let routes = routes
            .into_iter()
            .map(|(name, pattern)| (name, Route::parse(&pattern)))
            .collect();

        Self { routes }
    }

    /// Safe version: returns an error when the route name is unknown
    /// or the parameter count does not match.
    pub fn try_url(&self, name: &str, params: &[&dyn Display]) -> Result<String> {
        self.build(name, params).map_err(|e| match e {
            RouteError::InvalidName => anyhow!("invalid route name `{name}`"),
            RouteError::InvalidParams { expected } => anyhow!(
                "route `{name}` expects {expected} parameter(s), got {}",
                params.len()
            ),
        })
    }

    /// Safe version for routes without parameters.
    pub fn try_url0(&self, name: &str) -> Result<String> {
        self.try_url(name, &[])
    }

    /// Non-failing version: returns `#invalid-route-name` or
    /// `#invalid-parameters` instead of an error (handy in templates).
    pub fn url(&self, name: &str, params: &[&dyn Display]) -> String {
        self.build(name, params)
            .unwrap_or_else(|e| e.fragment().to_string())
    }

    /// Non-failing version for routes without parameters.
    pub fn url0(&self, name: &str) -> String {
        self.url(name, &[])
    }

    /// Shared implementation: validates input, then interleaves the
    /// literal parts with the percent-encoded parameters.
    fn build(&self, name: &str, params: &[&dyn Display]) -> Result<String, RouteError> {
        // Unknown route name
        let route = self.routes.get(name).ok_or(RouteError::InvalidName)?;

        // Wrong number of parameters
        if params.len() != route.param_count() {
            return Err(RouteError::InvalidParams {
                expected: route.param_count(),
            });
        }

        // One allocation in the common case (16 bytes guessed per parameter)
        let mut url = String::with_capacity(route.literal_len + params.len() * 16);
        url.push_str(&route.parts[0]);

        for (param, part) in params.iter().zip(&route.parts[1..]) {
            // Same contract as `ToString`: only a broken Display impl can fail here
            write!(Encoder(&mut url), "{param}").expect("Display impl returned an error");
            url.push_str(part);
        }

        Ok(format!("{}{}", CONFIG.app.url, url))
    }
}

/// Builds a URL (safe version, returns `Result`) for a named route.
///
/// Usage: `route!("todos.show", user.email, todo.id)?`
/// Parameters can be any `Display` type, so no manual `to_string()` is needed.
#[macro_export]
macro_rules! route {
    ($name:expr $(,)?) => {
        ROUTE_MANAGER.try_url0($name)
    };
    ($name:expr, $($param:expr),+ $(,)?) => {
        ROUTE_MANAGER.try_url($name, &[$(&($param) as &dyn ::std::fmt::Display),+])
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;

    /// Placeholder for your startup-generated map.
    fn generated_routes() -> HashMap<String, String> {
        HashMap::from([
            (
                "todos.show".to_string(),
                "/todos/show/{user}/{id}".to_string(),
            ),
            ("about".to_string(), "/about".to_string()),
        ])
    }

    /// Global route manager, built on first access.
    pub static ROUTE_MANAGER: LazyLock<RouteManager> =
        LazyLock::new(|| RouteManager::new(generated_routes()));

    #[test]
    fn route_manager_fallback_api() {
        let m = &*ROUTE_MANAGER;
        assert_eq!(
            m.url("todos.show", &[&"a b", &"1/2"]),
            format!("{}{}", CONFIG.app.url, "/todos/show/a%20b/1%2F2")
        );
        assert_eq!(m.url0("about"), format!("{}{}", CONFIG.app.url, "/about"));
        assert_eq!(m.url0("nope"), "#invalid-route-name");
        assert_eq!(m.url("todos.show", &[&1]), "#invalid-parameters");
    }

    #[test]
    fn route_manager_safe_api_and_macro() {
        let email = String::from("a@b.com");
        let id = 42;

        assert_eq!(
            route!("todos.show", email, id).unwrap(),
            format!("{}{}", CONFIG.app.url, "/todos/show/a%40b.com/42")
        );
        assert_eq!(
            route!("about").unwrap(),
            format!("{}{}", CONFIG.app.url, "/about")
        );
        assert!(route!("nope").is_err());
        assert!(route!("todos.show", id).is_err());
    }
}
