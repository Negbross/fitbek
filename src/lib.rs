pub mod app;
pub mod config;
pub mod routes;

#[cfg(feature = "hydrate")]
mod hydrate {
    use crate::routes::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
        console_error_panic_hook::set_once();
        leptos::mount::hydrate_body(App);
    }
}
