use crate::app::repositories::feedback_repository::FeedbackRepository;
use crate::app::utils::error::AppError;
use entity::feedbacks;
use sea_orm::DatabaseConnection;

pub struct FeedbackService;

impl FeedbackService {
    pub async fn get_feedbacks(db: &DatabaseConnection) -> Result<Vec<feedbacks::Model>, AppError> {
        // Business logic could go here (e.g. filtering, auth checks)
        FeedbackRepository::get_all(db).await
    }

    pub async fn submit_feedback(
        db: &DatabaseConnection,
        payload: crate::app::models::feedback::SubmitFeedbackPayload,
    ) -> Result<feedbacks::Model, AppError> {
        use validator::Validate;
        payload
            .validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        FeedbackRepository::insert(db, payload.content).await
    }
}
