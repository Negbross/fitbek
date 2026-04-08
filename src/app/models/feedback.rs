use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FeedbackDto {
    pub id: Uuid,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct SubmitFeedbackPayload {
    #[validate(length(min = 5, message = "Feedback must be at least 5 characters long."))]
    pub content: String,
}
#[cfg(feature = "ssr")]
impl From<entity::feedbacks::Model> for FeedbackDto {
    fn from(model: entity::feedbacks::Model) -> Self {
        Self {
            id: model.id,
            content: model.content,
            created_at: model.created_at,
            user_id: model.user_id,
        }
    }
}
