use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use handlebars::{DirectorySourceOptions, Handlebars, TemplateError};

pub type TemplateEngine = Handlebars<'static>;

pub fn build_template_engine() -> Result<TemplateEngine, TemplateError> {
    let mut handlebars = Handlebars::new();
    let options = DirectorySourceOptions::default();

    handlebars.register_templates_directory("templates/", options)?;

    Ok(handlebars)
}

pub fn render_to_response(
    handlebars: &TemplateEngine,
    template_name: &str,
    data: &impl serde::Serialize,
) -> Response {
    match handlebars.render(template_name, data) {
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
