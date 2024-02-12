use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use handlebars::{DirectorySourceOptions, Handlebars};

pub type TemplateEngine = Arc<Handlebars<'static>>;

pub fn create_engine() -> TemplateEngine {
    let mut handlebars = Handlebars::new();
    let options = DirectorySourceOptions::default();

    handlebars
        .register_templates_directory("templates/", options)
        .unwrap();

    Arc::new(handlebars)
}

pub fn render_response(
    template_engine: &TemplateEngine,
    template_name: &str,
    data: &impl serde::Serialize,
) -> Response {
    let contents = template_engine.render(template_name, data);
    match contents {
        Ok(contents) => Html(contents).into_response(),
        Err(e) => {
            tracing::error!("Failed to render template: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to render template",
            )
                .into_response()
        }
    }
}
