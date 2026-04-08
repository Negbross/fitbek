pub mod auth;
#[cfg(feature = "ssr")]
pub mod error;
pub mod format_date;

pub async fn rate_limiter(_throttle: i32) {}
