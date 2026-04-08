use crate::app::models::feedback::FeedbackDto;
use leptos::prelude::*;

#[server(SubmitFeedback, "/api")]
pub async fn submit_feedback(content: String) -> Result<(), ServerFnError> {
    use crate::app::services::feedback_service::FeedbackService;
    use axum::extract::Extension;
    use sea_orm::DatabaseConnection;

    let Extension(db) = leptos_axum::extract::<Extension<DatabaseConnection>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let payload = crate::app::models::feedback::SubmitFeedbackPayload { content };

    FeedbackService::submit_feedback(&db, payload)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[server(GetFeedbacks, "/api")]
pub async fn get_feedbacks() -> Result<Vec<FeedbackDto>, ServerFnError> {
    use crate::app::services::feedback_service::FeedbackService;
    use crate::app::utils::auth::validate_jwt;
    use crate::config::config::Config;
    use axum::extract::Extension;
    use axum_extra::extract::cookie::CookieJar;
    use sea_orm::DatabaseConnection;

    let headers = leptos_axum::extract::<http::HeaderMap>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let cookies = CookieJar::from_headers(&headers);
    let token = cookies.get("jwt").map(|c| c.value());

    if let Some(t) = token {
        let secret = Config::init().jwt_secret;
        if validate_jwt(t, &secret).is_err() {
            return Err(ServerFnError::new(crate::app::utils::error::AppError::Unauthorized("Invalid token".to_string()).to_string()));
        }
    } else {
        return Err(ServerFnError::new(crate::app::utils::error::AppError::Unauthorized("Missing token".to_string()).to_string()));
    }

    let Extension(db) = leptos_axum::extract::<Extension<DatabaseConnection>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let feedbacks = FeedbackService::get_feedbacks(&db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let dtos = feedbacks
        .into_iter()
        .map(|f| FeedbackDto {
            id: f.id,
            content: f.content,
            created_at: f.created_at,
            user_id: f.user_id,
        })
        .collect();

    Ok(dtos)
}
