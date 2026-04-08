#[cfg(feature = "ssr")]
use axum::Router;
#[cfg(feature = "ssr")]
use fitbek::routes::*;
#[cfg(feature = "ssr")]
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_axum::{generate_route_list, LeptosRoutes};

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::extract::Extension;
    use leptos::config::get_configuration;
    use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_max_level(tracing::Level::INFO)
        .init();

    let conf = get_configuration(None).unwrap();
    let mut leptos_options = conf.leptos_options;
    
    let config = fitbek::config::config::Config::init();
    let addr: std::net::SocketAddr = format!("{}:{}", config.host, config.port).parse().expect("Invalid HOST or PORT in .env");
    leptos_options.site_addr = addr;
    
    let routes = generate_route_list(App);

    // Setup Rate Limiter
    let governor_conf = std::sync::Arc::new(
        GovernorConfigBuilder::default()
            .per_millisecond(1000) // 1 request per second
            .burst_size(5) // Allow up to 5 requests in a burst
            .finish()
            .unwrap(),
    );

    let db = fitbek::config::database::setup_db(&config)
        .await
        .expect("Failed to setup database");

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            || {}, // DB is provided via Axum Extension layer below, not Leptos context
            {
                let leptos_options = leptos_options.clone();
                move || {
                    let options = leptos_options.clone();
                    view! { <Shell options/> }
                }
            },
        )
        .fallback(leptos_axum::file_and_error_handler({
            let options = leptos_options.clone();
            move |_| {
                let options = options.clone();
                view! { <Shell options/> }
            }
        }))
        .layer(GovernorLayer::new(governor_conf.clone()))
        .layer(Extension(db))
        .with_state(leptos_options);

    tracing::info!("Listening on http://{}", &addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // Required for `cargo leptos build` in hydrate mode
}
