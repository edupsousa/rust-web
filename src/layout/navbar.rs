use serde::Serialize;

#[derive(Serialize)]
pub struct NavbarTemplateData {
    login_visible: bool,
    signup_visible: bool,
    logout_visible: bool,
    private_visible: bool,
}

impl NavbarTemplateData {
    pub fn new(is_signed_in: bool) -> Self {
        Self {
            login_visible: !is_signed_in,
            signup_visible: !is_signed_in,
            logout_visible: is_signed_in,
            private_visible: is_signed_in,
        }
    }
}
