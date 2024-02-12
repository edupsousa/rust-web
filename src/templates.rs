use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use handlebars::{DirectorySourceOptions, Handlebars, TemplateError};

#[derive(Clone)]
pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
}

impl TemplateEngine {
    pub fn build() -> Result<TemplateEngine, TemplateError> {
        let mut handlebars = Handlebars::new();
        let options = DirectorySourceOptions::default();

        handlebars.register_templates_directory("templates/", options)?;

        Ok(TemplateEngine { handlebars })
    }

    pub fn render(
        &self,
        name: &str,
        data: &impl serde::Serialize,
    ) -> Result<String, handlebars::RenderError> {
        self.handlebars.render(name, data)
    }

    pub fn render_response(
        &self,
        template_name: &str,
        data: &impl serde::Serialize,
    ) -> Response {
        match self.render(template_name, data) {
            Ok(contents) => Html(contents).into_response(),
            Err(e) => {
                tracing::error!("Failed to render template: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to render template",
                ).into_response()
            }
        }
    }
}
