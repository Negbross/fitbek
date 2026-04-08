// Placeholder for user related server functions
// e.g. Login, Logout, etc.
use leptos::prelude::*;

#[server(Login, "/api")]
pub async fn login() -> Result<(), ServerFnError> {
    // Logic will go here
    Ok(())
}
