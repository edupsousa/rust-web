use std::time::SystemTime;

use async_trait::async_trait;
use entity::session;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;
use tower_sessions::{
    cookie::time::OffsetDateTime,
    session::{Id, Record},
    session_store, ExpiredDeletion, SessionStore,
};

#[derive(Debug, Clone)]
pub struct DatabaseSessionStore {
    db: DatabaseConnection,
}

impl DatabaseSessionStore {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ExpiredDeletion for DatabaseSessionStore {
    async fn delete_expired(&self) -> session_store::Result<()> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i32;

        session::Entity::delete_many()
            .filter(session::Column::Expiry.lt(now))
            .exec(&self.db)
            .await
            .unwrap();

        Ok(())
    }
}

#[async_trait]
impl SessionStore for DatabaseSessionStore {
    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let new_session = session::ActiveModel {
            id: Set(record.id.to_string()),
            data: Set(json!(record.data).to_string()),
            expiry: Set(record.expiry_date.unix_timestamp().try_into().unwrap()),
        };

        match session::Entity::find_by_id(record.id.to_string()).one(&self.db).await.unwrap() {
            Some(_) => {
                session::Entity::update(new_session)
                    .filter(session::Column::Id.eq(record.id.to_string()))
                    .exec(&self.db)
                    .await
                    .unwrap();
            }
            None => {
                session::Entity::insert(new_session)
                    .exec(&self.db)
                    .await
                    .unwrap();
            }
        }

        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let session = session::Entity::find()
            .filter(session::Column::Id.eq(session_id.to_string()))
            .one(&self.db)
            .await
            .unwrap()
            .unwrap();
        let data = serde_json::from_str(&session.data).unwrap();
        let record = Record {
            id: *session_id,
            data,
            expiry_date: OffsetDateTime::from_unix_timestamp(session.expiry as i64).unwrap(),
        };
        Ok(Some(record))
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        session::Entity::delete_by_id(session_id.to_string())
            .exec(&self.db)
            .await
            .unwrap();
        Ok(())
    }
}
