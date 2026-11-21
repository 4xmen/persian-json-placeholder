use crate::AppState;
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{Html, IntoResponse, Response},
};
use minijinja::context;

pub async fn error_pages_middleware(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let response = next.run(req).await;

    let status = response.status();

    // Only handle error status codes
    if status.is_success() || status.is_redirection() {
        return response;
    }

    // Optional: if the response already has a body (you manually returned jinja Html),
    // you can decide to keep it or still override.
    // For now we always override error pages to keep them consistent.

    let (template_name, title) = match status {
        StatusCode::NOT_FOUND => ("errors/404.j2", "Page Not Found"),
        StatusCode::FORBIDDEN => ("errors/403.j2", "Forbidden"),
        StatusCode::UNAUTHORIZED => ("errors/401.j2", "Unauthorized"),
        StatusCode::INTERNAL_SERVER_ERROR => ("errors/500.j2", "Internal Server Error"),
        StatusCode::BAD_REQUEST => ("errors/400.j2", "Bad Request"),
        _ => ("errors/generic.j2", "Error"),
    };

    let html = match state.templates.get_template(template_name) {
        Ok(tmpl) => {
            tmpl.render(context! {
                title => title,
                status => status.as_u16(),
                // You can later pass more data if needed
            })
            .unwrap_or_else(|_| fallback_html(status, title))
        }
        Err(_) => fallback_html(status, title),
    };

    (status, Html(html)).into_response()
}

fn fallback_html(status: StatusCode, title: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
        <html>
        <head><title>{}</title></head>
        <body>
            <h1>{}</h1>
            <p>Status: {}</p>
        </body>
        </html>"#,
        title,
        title,
        status.as_u16()
    )
}
