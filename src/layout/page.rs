
use serde::Serialize;
use super::navbar::NavbarTemplateData;

#[derive(Serialize)]
pub struct PageTemplateData<T> 
where T: Serialize
{
  navbar: NavbarTemplateData,
  content: T,
}

impl<T> PageTemplateData<T> 
where T: Serialize
{
  pub fn new(is_signed_in: bool, content: T) -> Self {
    Self {
      navbar: NavbarTemplateData::new(is_signed_in),
      content,
    }
  }
}