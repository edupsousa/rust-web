use serde::Serialize;


#[derive(Serialize)]
pub struct PageMessage {
  pub class: &'static str,
  pub text: String,
}

impl From<axum_messages::Message> for PageMessage {
  fn from(message: axum_messages::Message) -> Self {
    let class = match message.level {
      axum_messages::Level::Error => "is-danger",
      axum_messages::Level::Warning => "is-warning",
      axum_messages::Level::Info => "is-info",
      axum_messages::Level::Success => "is-success",
      axum_messages::Level::Debug => "",
    };
    let text = message.message.to_string();

    Self {
      class,
      text,
    }
  }
}

pub type PageMessages = Vec<PageMessage>;