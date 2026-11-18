use axum::{Extension, extract::State, response::IntoResponse};
use std::time::Instant;
// use minijinja::functions::debug;
use rustavel_core::config::CONFIG;
use rustavel_core::helper::datetime::format_duration;
use rustavel_core::state::AppState;
use serde::Serialize;

#[derive(Serialize)]
struct SampleData {
    title: String,
    elapsed_time: Option<String>,
    url: String,
}

/// Renders the welcome page using MiniJinja.
pub async fn index(
    State(state): State<AppState>,
    Extension(start): Extension<Instant>,
) -> impl IntoResponse {
    // Prepare the data passed to the template.

    let elapsed_ms = format_duration(start.elapsed());

    let sample = SampleData {
        title: CONFIG.app.name.clone(),
        elapsed_time: Some(elapsed_ms),
        url: CONFIG.app.url.clone(),
    };

    state.view_with("welcome.j2", sample)
}
