use entity::{feedbacks, feedbacks::Entity as Feedbacks};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder, Set};
use crate::app::utils::error::AppError;

pub struct FeedbackRepository;

impl FeedbackRepository {
    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<feedbacks::Model>, AppError> {
        Feedbacks::find()
            .order_by_desc(feedbacks::Column::CreatedAt)
            .all(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn insert(
        db: &DatabaseConnection,
        content: String,
    ) -> Result<feedbacks::Model, AppError> {
        let new_feedback = feedbacks::ActiveModel {
            content: Set(content),
            created_at: Set(chrono::Utc::now().naive_utc()),
            user_id: Set(None),
            ..Default::default()
        };
        Ok(new_feedback.insert(db).await?)
    }
}
