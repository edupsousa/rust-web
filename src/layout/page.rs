use crate::templates::{render_to_response, TemplateEngine};

use super::{
    messages::{PageMessage, PageMessages},
    navbar::NavbarTemplateData,
};
use axum::response::Response;
use axum_messages::Messages;
use serde::Serialize;

#[derive(Serialize)]
pub struct PageTemplate<T>
where
    T: Serialize,
{
    navbar: NavbarTemplateData,
    content: T,
    messages: PageMessages,
    content_template: &'static str,
}

impl<T> PageTemplate<T>
where
    T: Serialize,
{
    pub fn new(content_template: &'static str, is_signed_in: bool, content: T) -> Self {
        Self {
            navbar: NavbarTemplateData::new(is_signed_in),
            content,
            messages: vec![],
            content_template,
        }
    }

    pub fn new_with_messages(
        content_template: &'static str,
        is_signed_in: bool,
        content: T,
        messages: Messages,
    ) -> Self {
        let messages = messages.into_iter().map(PageMessage::from).collect();

        Self {
            navbar: NavbarTemplateData::new(is_signed_in),
            content,
            messages,
            content_template,
        }
    }

    pub fn render(&self, template_engine: &TemplateEngine) -> Response {
        render_to_response(template_engine, "layout/page", self)
    }
}
