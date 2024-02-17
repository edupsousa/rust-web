use axum::{extract::{Request, State}, middleware::Next, response::Response};

use crate::{app::AppState, auth::layer::AuthSession, templates::TemplateEngine};

use super::{messages::{MessageLevel, PageMessage, PageMessages}, page_template::PageTemplate};


pub async fn with_layout_middleware(
  State(app_state): State<AppState>,
  auth_session: AuthSession,
  mut request: Request,
  next: Next
) -> Response {
  let template_engine = app_state.template_engine.clone();
  let is_signed_in = auth_session.user.is_some();
  let layout_middleware = LayoutMiddleware::new(template_engine, is_signed_in);
  request.extensions_mut().insert(layout_middleware);
  
  next.run(request).await
}

#[derive(Clone)]
pub struct LayoutMiddleware {
  template_engine: TemplateEngine,
  is_signed_in: bool,
  messages: Option<PageMessages>,
}

impl LayoutMiddleware {
  fn new(
    template_engine: TemplateEngine,
    is_signed_in: bool,
  ) -> Self {
    Self {
      template_engine,
      is_signed_in,
      messages: None,
    }
  }

  fn push_message(&mut self, message: PageMessage) {
    if self.messages.is_none() {
      self.messages = Some(PageMessages::new());
    }
    self.messages.as_mut().unwrap().add(message);
  }

  pub fn add_error_message(&mut self, text: &str) {
    self.push_message(PageMessage::new(MessageLevel::Error, text));
  }

  pub fn add_success_message(&mut self, text: &str) {
    self.push_message(PageMessage::new(MessageLevel::Success, text));
  }

  pub fn render(&self, template_name: &'static str, content: impl serde::Serialize) -> Response {
    PageTemplate::builder(template_name)
      .content(content)
      .navbar(self.is_signed_in)
      .maybe_messages(self.messages.clone())
      .build()
      .render(&self.template_engine)
  }
}