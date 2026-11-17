use crate::config::database::DbPoolType;
use crate::http::session::Session;
use axum::extract::FromRef;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use minijinja::value::Serde;
use minijinja::{Environment, Value, context};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug, FromRef)]
pub struct AppState {
    pub routes: Arc<HashMap<String, String>>,
    pub templates: Arc<Environment<'static>>,
    pub db_pool: DbPoolType,
}

impl AppState {
    /// Get route url by route name return str
    /// if the route exsits return
    /// otherwise return #route-name-not-found
    pub fn route(&self, route: &str) -> &str {
        match self.routes.get(route) {
            Some(route) => route,
            None => "#route-name-not-found",
        }
    }

    /// Renders a MiniJinja template with an empty context and returns an HTML response.
    ///
    /// Returns `200 OK` when rendering succeeds, or `500 Internal Server Error`
    /// if the template cannot be found or rendered.
    pub fn view(&self, view_file: &str) -> impl IntoResponse {
        let result = self
            .templates
            .get_template(view_file)
            .and_then(|template| template.render(()));

        match result {
            Ok(html) => (StatusCode::OK, Html(html)).into_response(),
            Err(error) => {
                tracing::error!("template `{view_file}` failed: {error:#}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }

    /// Renders a MiniJinja template with the provided data as its context.
    ///
    /// Returns `200 OK` when rendering succeeds, or `500 Internal Server Error`
    /// if the template cannot be found or rendered.
    pub fn view_with<T>(&self, view_file: &str, data: T) -> impl IntoResponse + use<T>
    where
        T: Serialize,
    {
        let result = self
            .templates
            .get_template(view_file)
            .and_then(|template| template.render(Serde(&data)));

        match result {
            Ok(html) => (StatusCode::OK, Html(html)).into_response(),
            Err(error) => {
                tracing::error!("template `{view_file}` failed: {error:#}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }

    /// Renders a template with the session's CSRF token injected automatically.
    /// Every page that contains a form or fetch call should be rendered with this.
    pub fn protected_view(
        &self,
        view_file: &str,
        ctx: Value,
        session: &Session,
    ) -> Result<Html<String>, StatusCode> {
        let full = context! { csrf_token => session.csrf_token(), ..ctx };

        self.templates
            .get_template(view_file)
            .and_then(|t| t.render(full))
            .map(Html)
            .map_err(|e| {
                tracing::error!("template `{view_file}` failed: {e:#}");
                StatusCode::INTERNAL_SERVER_ERROR
            })
    }
}
