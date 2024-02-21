use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct NavbarTemplateData {
    login_visible: bool,
    signup_visible: bool,
    logout_visible: bool,
}

impl NavbarTemplateData {
    pub fn new(is_signed_in: bool) -> Self {
        Self {
            login_visible: !is_signed_in,
            signup_visible: !is_signed_in,
            logout_visible: is_signed_in,
        }
    }
}

impl Default for NavbarTemplateData {
    fn default() -> Self {
        Self::new(false)
    }
}
