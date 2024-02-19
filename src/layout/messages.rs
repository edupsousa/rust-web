use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct PageMessage {
    pub class: &'static str,
    pub text: String,
}

pub enum MessageLevel {
    Error,
    Success,
}

impl PageMessage {
    pub fn new(level: MessageLevel, text: impl Into<String>) -> PageMessage {
        let class = match level {
            MessageLevel::Error => "is-danger",
            MessageLevel::Success => "is-success",
        };
        PageMessage {
            class,
            text: text.into(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct PageMessages(Vec<PageMessage>);

impl PageMessages {
    pub fn new() -> PageMessages {
        PageMessages(Vec::new())
    }

    pub fn add(&mut self, message: PageMessage) {
        self.0.push(message);
    }
}
