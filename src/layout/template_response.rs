use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response}, Extension,
};
use serde::Serialize;
use serde_json::Value;

use crate::{app::AppState, auth::layer::AuthSession};

use super::{messages::{MessageLevel, PageMessage, PageMessages}, page_template::PageTemplate};

#[derive(Clone)]
pub struct TemplateResponse {
    partial_name: String,
    content: Option<Value>,
    messages: Option<PageMessages>,
}

impl TemplateResponse {
    pub fn new(partial_name: impl Into<String>) -> Self {
        Self {
            partial_name: partial_name.into(),
            content: None,
            messages: None,
        }
    }

    pub fn content(mut self, content: impl Serialize) -> Self {
        self.content = serde_json::to_value(content).ok();
        self
    }

    fn push_message(&mut self, level: MessageLevel, text: impl Into<String>) {
        let messages = self.messages.get_or_insert_with(PageMessages::new);
        messages.add(PageMessage::new(level, text));
    }

    pub fn add_success_message(mut self, message: impl Into<String>) -> Self {
        self.push_message(MessageLevel::Success, message);
        self
    }

    pub fn add_error_message(mut self, message: impl Into<String>) -> Self {
        self.push_message(MessageLevel::Error, message);
        self
    }
}

impl IntoResponse for TemplateResponse {
    fn into_response(self) -> Response {
        Extension(self).into_response()
    }
}

pub async fn with_template_response(
    State(app_state): State<AppState>,
    auth_session: AuthSession,
    request: Request,
    next: Next,
) -> Response {
    let response = next.run(request).await;
    let response = match response.extensions().get::<TemplateResponse>() {
        Some(template_response) => {
            let template_engine = app_state.template_engine;
            let is_signed_in = auth_session.user.is_some();
            return PageTemplate::builder(template_response.partial_name.clone())
                .maybe_content(template_response.content.clone())
                .navbar(is_signed_in)
                .maybe_messages(template_response.messages.clone())
                .build()
                .render(&template_engine);
        }
        None => response,
    };

    response
}
