use crate::app::repositories::user_repository::UserRepository;
use sea_orm::{DatabaseConnection, DbErr};
use crate::app::utils::error::AppError;

pub struct UserService;

impl UserService {
    pub async fn ensure_admin_exists(db: &DatabaseConnection) -> Result<(), AppError> {
        let admin_exists = UserRepository::find_admin(db).await?;
        if admin_exists.is_none() {
            let hashed_password = bcrypt::hash("admin", bcrypt::DEFAULT_COST)
                .map_err(|e| AppError::InternalError(e.to_string()))?;
            UserRepository::create_admin(db, "admin".to_string(), hashed_password).await?;
            tracing::info!("Seeded initial admin user.");
        }
        Ok(())
    }

    pub async fn find_by_username(
        db: &DatabaseConnection,
        username: &str,
    ) -> Result<Option<entity::users::Model>, AppError> {
        UserRepository::find_by_username(db, username).await
    }
}
