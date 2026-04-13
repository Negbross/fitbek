use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FormDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub url_slug: String,
    pub created_at: Option<String>,
    pub questions: Vec<FormQuestionDto>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FormListItemDto {
    pub id: String,
    pub title: String,
    pub url_slug: String,
    pub response_count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FormQuestionDto {
    pub id: String,
    pub form_id: String,
    pub question_type: String,
    pub label: String,
    pub options: Option<String>,
    pub is_required: bool,
    pub order_index: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FormResponseDto {
    pub id: String,
    pub form_id: String,
    pub submitted_at: Option<String>,
    pub answers: Vec<FormAnswerDto>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FormAnswerDto {
    pub id: String,
    pub question_id: String,
    pub question_label: Option<String>,
    pub answer_value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateFormPayload {
    pub title: String,
    pub description: Option<String>,
    pub questions: Vec<CreateFormQuestionPayload>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateFormQuestionPayload {
    pub question_type: String,
    pub label: String,
    pub options: Option<String>,
    pub is_required: bool,
    pub order_index: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateFormPayload {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubmitResponsePayload {
    pub form_id: String,
    pub answers: Vec<SubmitAnswerPayload>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubmitAnswerPayload {
    pub question_id: String,
    pub answer_value: Option<String>,
}
