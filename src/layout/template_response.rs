use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Extension,
};
use serde::Serialize;
use serde_json::Value;

use crate::{app::AppState, auth::layer::AuthSession};

use super::{
    messages::{MessageLevel, PageMessage, PageMessages},
    page_template::PageTemplate,
};

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

struct TemplateStateWrapper {
    app_state: AppState,
    auth_session: AuthSession,
    template_response: TemplateResponse,
}

impl IntoResponse for TemplateStateWrapper {
    fn into_response(self) -> Response {
        let template_engine = self.app_state.template_engine;
        let is_signed_in = self.auth_session.user.is_some();
        PageTemplate::builder(self.template_response.partial_name)
            .maybe_content(self.template_response.content)
            .navbar(is_signed_in)
            .maybe_messages(self.template_response.messages)
            .build()
            .render(&template_engine)
    }
}

pub async fn with_template_response(
    State(app_state): State<AppState>,
    auth_session: AuthSession,
    response: Response,
) -> Response {
    let response = match response.extensions().get::<TemplateResponse>() {
        Some(template_response) => {
            let template_response = template_response.to_owned();
            TemplateStateWrapper {
                app_state,
                auth_session,
                template_response,
            }.into_response()
        }
        None => response,
    };

    response
}
