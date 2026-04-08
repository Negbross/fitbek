use leptos::prelude::*;

#[server(Login, "/api")]
pub async fn login(username: String, password: String) -> Result<(), ServerFnError> {
    use crate::app::services::user_service::UserService;
    use crate::app::utils::auth::{create_jwt, set_auth_cookie};
    use crate::config::config::Config;
    use axum::extract::Extension;
    use sea_orm::DatabaseConnection;

    let Extension(db) = leptos_axum::extract::<Extension<DatabaseConnection>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Find user by username
    let user = UserService::find_by_username(&db, &username)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // If user missing or password doesn't match
    let valid = if let Some(u) = user {
        bcrypt::verify(&password, &u.password).unwrap_or(false)
    } else {
        false
    };

    if !valid {
        return Err(ServerFnError::new(crate::app::utils::error::AppError::InvalidCredentials.to_string()));
    }

    let jwt_secret = Config::init().jwt_secret;
    let token =
        create_jwt(&username, &jwt_secret).map_err(|e| ServerFnError::new(crate::app::utils::error::AppError::InternalError("Failed to create JWT".to_string()).to_string()))?;

    // Set cookie
    let cookie = set_auth_cookie(token);

    let resp = expect_context::<leptos_axum::ResponseOptions>();
    resp.insert_header(
        http::header::SET_COOKIE,
        http::header::HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );

    Ok(())
}

#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    use crate::app::utils::auth::clear_auth_cookie;
    let cookie = clear_auth_cookie();

    let resp = expect_context::<leptos_axum::ResponseOptions>();
    resp.insert_header(
        http::header::SET_COOKIE,
        http::header::HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );

    Ok(())
}
