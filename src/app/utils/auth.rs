#[cfg(feature = "ssr")]
use axum_extra::extract::cookie::Cookie;
#[cfg(feature = "ssr")]
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // username
    pub exp: usize,  // expiration time
}

#[cfg(feature = "ssr")]
pub fn create_jwt(username: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

#[cfg(feature = "ssr")]
pub fn validate_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

#[cfg(feature = "ssr")]
pub fn set_auth_cookie(token: String) -> Cookie<'static> {
    Cookie::build(("jwt", token))
        .path("/")
        .http_only(true)
        .secure(false) // Set to true if using HTTPS in production
        .same_site(cookie::SameSite::Lax)
        .build()
}

#[cfg(feature = "ssr")]
pub fn clear_auth_cookie() -> Cookie<'static> {
    Cookie::build(("jwt", ""))
        .path("/")
        .http_only(true)
        .secure(false)
        .same_site(cookie::SameSite::Lax)
        .max_age(time::Duration::ZERO)
        .build()
}
