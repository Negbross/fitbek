use std::fmt::Display;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response, Json};
use sea_orm::DbErr;
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    DbError(DbErr),
    NotFound,
    InvalidCredentials,
    Validation(String),
    DuplicateEntry(String),
    InternalError(String),
    InvalidToken,
    AlreadyLogin,
    Forbidden,
    BadRequest(String),
    FileNotFound,
    Unauthorized(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::DbError(err) => write!(f, "Kesalahan database: {}", err),
            AppError::NotFound => write!(f, "Data yang dicari tidak ditemukan"),
            AppError::InvalidCredentials => write!(f, "Kredensial tidak valid"),
            AppError::Validation(msg) => write!(f, "Error validasi: {}", msg),
            AppError::DuplicateEntry(msg) => write!(f, "Data sudah ada: {}", msg),
            AppError::InternalError(msg) => write!(f, "Kesalahan server: {}", msg),
            AppError::InvalidToken => write!(f, "Token tidak valid"),
            AppError::AlreadyLogin => write!(f, "Anda sudah login"),
            AppError::Forbidden => write!(f, "Akses ditolak"),
            AppError::BadRequest(msg) => write!(f, "Permintaan tidak valid: {}", msg),
            AppError::FileNotFound => write!(f, "File tidak ditemukan"),
            AppError::Unauthorized(msg) => write!(f, "Tidak terautentikasi: {}", msg),
        }
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> AppError {
        AppError::DbError(err)
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::DbError(err) => Some(err),
            _ => None,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DbError(err) =>
                match dotenvy::var("HOST_MODE").unwrap_or_default() == "development" {
                true => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Kesalahan database: {}", err)
                ),
                false => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Terjadi kesalahan pada server".to_string()
                )
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, "Data yang dicari tidak ditemukan".to_string()),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Kredensial tidak valid".to_string()),
            // Gunakan status 422 Unprocessable Entity untuk error validasi
            AppError::Validation(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            // Gunakan status 409 Conflict untuk data duplikat
            AppError::DuplicateEntry(message) => (StatusCode::CONFLICT, message),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "Kredensial tidak valid".to_string()),
            AppError::InternalError(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            AppError::AlreadyLogin => (StatusCode::FORBIDDEN, "Anda sudah login".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Belum Auth".to_string()),
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            AppError::FileNotFound => (StatusCode::NOT_FOUND, "File yang dicari gada".to_string()),
            AppError::Unauthorized(message) => (StatusCode::UNAUTHORIZED, message),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}