use crate::templates::{render_to_response, TemplateEngine};

use super::{messages::PageMessages, navbar::NavbarTemplateData};
use axum::response::Response;
use serde::Serialize;

#[derive(Serialize)]
pub struct PageTemplate<T>
where
    T: Serialize,
{
    navbar: NavbarTemplateData,
    content: Option<T>,
    messages: Option<PageMessages>,
    template_name: &'static str,
}

impl<T> PageTemplate<T>
where
    T: Serialize,
{
    pub fn builder(template_name: &'static str) -> PageTemplateBuilder<T> {
        PageTemplateBuilder::new(template_name)
    }

    pub fn render(&self, template_engine: &TemplateEngine) -> Response {
        render_to_response(template_engine, "layout/page", self)
    }
}

pub struct PageTemplateBuilder<T>
where
    T: Serialize,
{
    template_name: &'static str,
    content: Option<T>,
    navbar: Option<NavbarTemplateData>,
    messages: Option<PageMessages>,
}

impl<T> PageTemplateBuilder<T>
where
    T: Serialize,
{
    pub fn new(template_name: &'static str) -> Self {
        Self {
            template_name,
            content: None,
            navbar: None,
            messages: None,
        }
    }

    pub fn content(mut self, content: T) -> Self {
        self.content = Some(content);
        self
    }

    pub fn navbar(mut self, is_signed_in: bool) -> Self {
        self.navbar = Some(NavbarTemplateData::new(is_signed_in));
        self
    }

    pub fn maybe_messages(mut self, messages: Option<PageMessages>) -> Self {
        self.messages = messages;
        self
    }

    pub fn build(self) -> PageTemplate<T> {
        PageTemplate {
            navbar: self.navbar.unwrap_or_default(),
            content: self.content,
            messages: self.messages,
            template_name: self.template_name,
        }
    }
}
