use entity::{users, users::Entity as Users};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use crate::app::utils::error::AppError;

pub struct UserRepository;

impl UserRepository {
    pub async fn create_admin(
        db: &DatabaseConnection,
        username: String,
        password_hash: String,
    ) -> Result<users::Model, AppError> {
        let admin = users::ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            username: Set(username),
            role: Set("Admin".to_string()),
            password: Set(password_hash),
            ..Default::default()
        };
        Ok(admin.insert(db).await?)
    }

    pub async fn find_admin(db: &DatabaseConnection) -> Result<Option<users::Model>, AppError> {
        Ok(Users::find().one(db).await?)
    }

    pub async fn find_by_username(
        db: &DatabaseConnection,
        username: &str,
    ) -> Result<Option<users::Model>, AppError> {
        use sea_orm::ColumnTrait;
        use sea_orm::QueryFilter;
        Users::find()
            .filter(users::Column::Username.eq(username))
            .one(db)
            .await
            .map_err(AppError::from)
    }
}
