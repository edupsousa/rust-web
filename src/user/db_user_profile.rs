use entity::user_profile;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};

pub struct SaveUserProfileData {
  pub display_name: String,
}

pub async fn save_user_profile(
  db: &DatabaseConnection,
  user_id: i32,
  data: SaveUserProfileData,
) -> Result<(), DbErr> {
  let profile = user_profile::Entity::find_by_id(user_id).one(db).await?;
  match profile {
    Some(profile) => {
      let mut profile: user_profile::ActiveModel = profile.into();
      profile.display_name = Set(data.display_name);
      profile.update(db).await?;
      
      Ok(())
    }
    None => {
      let profile = user_profile::ActiveModel {
        id: Set(user_id),
        display_name: Set(data.display_name),
      };
      profile.insert(db).await?;
      
      Ok(())
    }
  }
}