use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserDto {
    pub id: Uuid,
    pub username: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct RegisterUserPayload {
    #[validate(length(
        min = 3,
        max = 20,
        message = "Username length should be between 3 and 20 characters."
    ))]
    pub username: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long."))]
    pub password: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct LoginUserPayload {
    #[validate(length(min = 1, message = "Username cannot be empty."))]
    pub username: String,
    #[validate(length(min = 1, message = "Password cannot be empty."))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginResponse {
    pub token: String,
    pub user_info: UserDto,
}

#[cfg(feature = "ssr")]
impl From<entity::users::Model> for UserDto {
    fn from(user_model: entity::users::Model) -> Self {
        Self {
            id: user_model.id,
            username: user_model.username,
            role: user_model.role,
        }
    }
}
