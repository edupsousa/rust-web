use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    Form,
};
use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::{
    app::AppState,
    auth::layer::AuthSession,
    layout::{messages::PageMessages, page_template::PageTemplate},
    templates::TemplateEngine,
};

use super::db_user_profile::{self, get_user_profile, GetUserProfileResult};

#[derive(Serialize, Deserialize, Default, Validate)]
pub struct ProfileForm {
    #[validate(length(min = 1, message = "Display name is required"))]
    display_name: String,
}

#[derive(Serialize, Default)]
pub struct ProfilePage {
    form: ProfileForm,
    errors: ValidationErrors,
}

fn render_profile_page(
    template_engine: &TemplateEngine,
    data: ProfilePage,
    messages: Option<PageMessages>,
) -> Response {
    PageTemplate::builder("user/profile")
        .content(serde_json::to_value(data).unwrap())
        .navbar(true)
        .maybe_messages(messages)
        .build()
        .render(template_engine)
}

impl From<GetUserProfileResult> for ProfileForm {
    fn from(profile: GetUserProfileResult) -> Self {
        ProfileForm {
            display_name: profile.display_name,
        }
    }
}

pub async fn get_profile_page(State(app): State<AppState>, auth_session: AuthSession) -> Response {
    let user = auth_session.user.unwrap();
    let form = match get_user_profile(&app.database_connection, user.id()).await {
        Some(profile) => profile.into(),
        None => ProfileForm::default(),
    };

    render_profile_page(
        &app.template_engine,
        ProfilePage {
            form,
            errors: ValidationErrors::default(),
        },
        None,
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
    let mut messages = PageMessages::new();
    let user = auth_session.user.unwrap();
    let user_id = user.id();

    match form.validate() {
        Ok(()) => {
            match db_user_profile::save_user_profile(&app.database_connection, user_id, form.into())
                .await
            {
                Ok(()) => {
                    messages.success("Profile updated");

                    Redirect::to("/user/profile").into_response()
                }
                Err(e) => {
                    tracing::error!("Failed to save user profile: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to save user profile",
                    )
                        .into_response()
                }
            }
        }
        Err(errors) => {
            render_profile_page(&app.template_engine, ProfilePage { form, errors }, None)
        }
    }
}
