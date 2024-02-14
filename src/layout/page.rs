use crate::templates::{render_to_response, TemplateEngine};

use super::navbar::NavbarTemplateData;
use axum::response::Response;
use axum_messages::{Level, Messages};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    class: String,
    text: String,
}

#[derive(Serialize)]
pub struct PageTemplateData<T>
where
    T: Serialize,
{
    navbar: NavbarTemplateData,
    content: T,
    messages: Vec<Message>,
    content_template: &'static str,
}

fn get_level_class(level: &Level) -> String {
    match level {
        Level::Error => "is-danger",
        Level::Warning => "is-warning",
        Level::Info => "is-info",
        Level::Success => "is-success",
        Level::Debug => "",
    }
    .to_string()
}

impl<T> PageTemplateData<T>
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

    pub fn new_with_messages(content_template: &'static str, is_signed_in: bool, content: T, messages: Messages) -> Self {
        let messages = messages
            .into_iter()
            .map(|message| Message {
                class: get_level_class(&message.level),
                text: message.message,
            })
            .collect::<Vec<_>>();

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
