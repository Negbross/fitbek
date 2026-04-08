#[cfg(feature = "ssr")]
use crate::config::config::Config;
#[cfg(feature = "ssr")]
use migration::{Migrator, MigratorTrait};
#[cfg(feature = "ssr")]
use sea_orm::{Database, DatabaseConnection, DbErr};

#[cfg(feature = "ssr")]
pub async fn setup_db(config: &Config) -> Result<DatabaseConnection, DbErr> {
    let db_url = &config.database_url;
    let db: DatabaseConnection = Database::connect(db_url).await?;

    // Run Migrations (this replaces manual table creation)
    Migrator::up(&db, None)
        .await
        .map_err(|e| DbErr::Custom(e.to_string()))?;

    // Seed the initial admin user if not exists
    crate::app::services::user_service::UserService::ensure_admin_exists(&db)
        .await
        .map_err(|e| DbErr::Custom(e.to_string()))?;

    Ok(db)
}
