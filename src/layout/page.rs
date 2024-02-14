use super::navbar::NavbarTemplateData;
use axum_messages::{Level, Messages};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    level: String,
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
}

fn get_level_value(level: &Level) -> String {
    match level {
        Level::Error => "error",
        Level::Warning => "warning",
        Level::Info => "info",
        Level::Success => "success",
        Level::Debug => "debug",
    }
    .to_string()
}

impl<T> PageTemplateData<T>
where
    T: Serialize,
{
    pub fn new(is_signed_in: bool, content: T) -> Self {
        Self {
            navbar: NavbarTemplateData::new(is_signed_in),
            content,
            messages: vec![],
        }
    }

    pub fn new_with_messages(is_signed_in: bool, content: T, messages: Messages) -> Self {
        let messages = messages
            .into_iter()
            .map(|message| Message {
                level: get_level_value(&message.level),
                text: message.message,
            })
            .collect::<Vec<_>>();

        Self {
            navbar: NavbarTemplateData::new(is_signed_in),
            content,
            messages,
        }
    }
}
