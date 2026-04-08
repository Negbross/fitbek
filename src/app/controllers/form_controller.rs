use leptos::prelude::*;
use crate::app::models::form::{
    CreateFormPayload, FormDto, FormListItemDto, FormResponseDto, SubmitResponsePayload,
};

#[server(CreateForm, "/api")]
pub async fn create_form(payload: CreateFormPayload) -> Result<String, ServerFnError> {
    use crate::app::services::form_service::FormService;
    use crate::app::services::user_service::UserService;
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

    let username = if let Some(t) = token {
        let secret = Config::init().jwt_secret;
        match validate_jwt(t, &secret) {
            Ok(claims) => claims.sub,
            Err(_) => return Err(ServerFnError::new(crate::app::utils::error::AppError::Unauthorized("Invalid token".to_string()).to_string())),
        }
    } else {
        return Err(ServerFnError::new(crate::app::utils::error::AppError::Unauthorized("Missing token".to_string()).to_string()));
    };

    let Extension(db) = leptos_axum::extract::<Extension<DatabaseConnection>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let user_model = UserService::find_by_username(&db, &username)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new(crate::app::utils::error::AppError::NotFound.to_string()))?;

    let slug = FormService::create_form(&db, user_model.id, payload)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(slug)
}

#[server(GetForms, "/api")]
pub async fn get_forms() -> Result<Vec<FormListItemDto>, ServerFnError> {
    use crate::app::services::form_service::FormService;
    use crate::app::services::user_service::UserService;
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

    let username = if let Some(t) = token {
        let secret = Config::init().jwt_secret;
        match validate_jwt(t, &secret) {
            Ok(claims) => claims.sub,
            Err(_) => return Err(ServerFnError::new(crate::app::utils::error::AppError::Unauthorized("Invalid token".to_string()).to_string())),
        }
    } else {
        return Err(ServerFnError::new(crate::app::utils::error::AppError::Unauthorized("Missing token".to_string()).to_string()));
    };

    let Extension(db) = leptos_axum::extract::<Extension<DatabaseConnection>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let user_model = UserService::find_by_username(&db, &username)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new(crate::app::utils::error::AppError::NotFound.to_string()))?;

    let dtos = FormService::get_forms(&db, user_model.id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(dtos)
}

#[server(GetFormBySlug, "/api")]
pub async fn get_form_by_slug(slug: String) -> Result<FormDto, ServerFnError> {
    use crate::app::services::form_service::FormService;
    use axum::extract::Extension;
    use sea_orm::DatabaseConnection;

    let Extension(db) = leptos_axum::extract::<Extension<DatabaseConnection>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let dto = FormService::get_form_by_slug(&db, slug)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new(crate::app::utils::error::AppError::NotFound.to_string()))?;

    Ok(dto)
}

#[server(SubmitResponse, "/api")]
pub async fn submit_response(payload: SubmitResponsePayload) -> Result<(), ServerFnError> {
    use crate::app::services::form_service::FormService;
    use axum::extract::Extension;
    use sea_orm::DatabaseConnection;

    let Extension(db) = leptos_axum::extract::<Extension<DatabaseConnection>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    FormService::submit_response(&db, payload)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[server(GetFormResponses, "/api")]
pub async fn get_form_responses(form_id: String) -> Result<Vec<FormResponseDto>, ServerFnError> {
    use crate::app::services::form_service::FormService;
    use crate::app::services::user_service::UserService;
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

    let username = if let Some(t) = token {
        let secret = Config::init().jwt_secret;
        match validate_jwt(t, &secret) {
            Ok(claims) => claims.sub,
            Err(_) => return Err(ServerFnError::new(crate::app::utils::error::AppError::Unauthorized("Invalid token".to_string()).to_string())),
        }
    } else {
        return Err(ServerFnError::new(crate::app::utils::error::AppError::Unauthorized("Missing token".to_string()).to_string()));
    };

    let Extension(db) = leptos_axum::extract::<Extension<DatabaseConnection>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let user_model = UserService::find_by_username(&db, &username)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new(crate::app::utils::error::AppError::NotFound.to_string()))?;

    let dtos = FormService::get_form_responses(&db, user_model.id, form_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new(crate::app::utils::error::AppError::Forbidden.to_string()))?;

    Ok(dtos)
}
