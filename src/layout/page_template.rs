use crate::templates::{render_to_response, TemplateEngine};

use super::{messages::PageMessages, navbar::NavbarTemplateData};
use axum::response::Response;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct PageTemplate
{
    navbar: NavbarTemplateData,
    content: Option<Value>,
    messages: Option<PageMessages>,
    template_name: String,
}

impl PageTemplate
{
    pub fn builder(template_name: impl Into<String>) -> PageTemplateBuilder {
        PageTemplateBuilder::new(template_name)
    }

    pub fn render(&self, template_engine: &TemplateEngine) -> Response {
        render_to_response(template_engine, "layout/page", self)
    }
}

pub struct PageTemplateBuilder
{
    template_name: String,
    content: Option<Value>,
    navbar: Option<NavbarTemplateData>,
    messages: Option<PageMessages>,
}

impl PageTemplateBuilder
{
    pub fn new(template_name: impl Into<String>) -> Self {
        Self {
            template_name: template_name.into(),
            content: None,
            navbar: None,
            messages: None,
        }
    }

    pub fn maybe_content(mut self, content: Option<Value>) -> Self {
        self.content = content;
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

    pub fn build(self) -> PageTemplate {
        PageTemplate {
            navbar: self.navbar.unwrap_or_default(),
            content: self.content,
            messages: self.messages,
            template_name: self.template_name,
        }
    }
}
