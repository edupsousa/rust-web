use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Form,
};
use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::{app::AppState, auth::layer::AuthSession, layout::template_response::TemplateResponse};

use super::db_user_profile::{self, get_user_profile, GetUserProfileResult};

#[derive(Serialize, Deserialize, Default, Validate, Clone)]
pub struct ProfileForm {
    #[validate(length(min = 1, message = "Display name is required"))]
    display_name: String,
}

#[derive(Serialize, Default)]
pub struct ProfilePage {
    form: ProfileForm,
    errors: ValidationErrors,
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

    TemplateResponse::new("user/profile")
        .content(ProfilePage {
            form,
            errors: ValidationErrors::default(),
        })
        .into_response()
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
    let response = TemplateResponse::new("user/profile");

    match form.validate() {
        Ok(()) => {
            match db_user_profile::save_user_profile(
                &app.database_connection,
                user_id,
                form.clone().into(),
            )
            .await
            {
                Ok(()) => response
                    .content(ProfilePage {
                        form,
                        errors: ValidationErrors::default(),
                    })
                    .add_success_message("Profile updated")
                    .into_response(),
                Err(e) => {
                    tracing::error!("Failed to save user profile: {:?}", e);

                    response
                        .content(ProfilePage {
                            form,
                            errors: ValidationErrors::default(),
                        })
                        .add_error_message("Failed to save user profile")
                        .into_response()
                }
            }
        }
        Err(errors) => response
            .content(ProfilePage { form, errors })
            .into_response(),
    }
}
