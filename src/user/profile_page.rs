use std::collections::HashMap;

use axum::{
    extract::State, http::StatusCode, response::{IntoResponse, Redirect, Response}, Form
};
use axum_login::AuthUser;
use serde::{Deserialize, Serialize};

use crate::{
    app::AppState,
    auth::layer::AuthSession,
    layout::page::PageTemplateData,
    templates::{render_to_response, TemplateEngine},
};

use super::db_user_profile;

#[derive(Serialize, Deserialize, Default)]
pub struct ProfileForm {
    display_name: String,
}

type FormErrors = HashMap<&'static str, &'static str>;

#[derive(Serialize, Default)]
pub struct ProfilePage {
    form: ProfileForm,
    errors: FormErrors,
}

fn render_profile_page(template_engine: &TemplateEngine, data: ProfilePage) -> Response {
    let page_data = PageTemplateData::new(true, data);
    render_to_response(template_engine, "user/profile", &page_data)
}

impl ProfileForm {
    fn get_errors(&self) -> FormErrors {
        let mut errors = FormErrors::default();

        if self.display_name.is_empty() {
            errors.insert("display_name", "Display name is required");
        }

        errors
    }
}

pub async fn get_profile_page(State(app): State<AppState>, auth_session: AuthSession) -> Response {
    let _user = auth_session.user.unwrap();

    let form = ProfileForm::default(); // TODO: Load from user profile

    render_profile_page(
        &app.template_engine,
        ProfilePage {
            form,
            errors: FormErrors::default(),
        },
    )
}

impl From<ProfileForm> for db_user_profile::SaveUserProfileData {
    fn from(form: ProfileForm) -> Self {
        db_user_profile::SaveUserProfileData {
            display_name: form.display_name,
        }
    }
}

pub async fn post_profile_page(
    State(app): State<AppState>,
    auth_session: AuthSession,
    Form(form): Form<ProfileForm>,
) -> Response {
    let user = auth_session.user.unwrap();
    let user_id = user.id();

    let errors = form.get_errors();

    if errors.is_empty() {
        match db_user_profile::save_user_profile(&app.database_connection, user_id, form.into()).await {
            Ok(()) => {
                return Redirect::to("/profile").into_response();
            }
            Err(e) => {
                tracing::error!("Failed to save user profile: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save user profile").into_response();
            }
        }
    }

    let data = ProfilePage { form, errors };

    render_profile_page(&app.template_engine, data)
}
