mod elgato;
mod error;
mod metrics;
mod routes;
mod state;
mod templates;

use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::routing::{get, post};
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

use elgato::ElgatoClient;
use state::{AppState, LightStatus};

const POLL_INTERVAL: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let light_host = std::env::var("ELGATO_HOST").unwrap_or_else(|_| "10.60.1.88".to_string());
    let light_port: u16 = std::env::var("ELGATO_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(9123);

    let state = AppState {
        client: ElgatoClient::new(&light_host, light_port),
        status: Arc::new(RwLock::new(LightStatus::default())),
        light_host: light_host.clone(),
    };

    tokio::spawn(poll_light(state.clone()));

    let app = Router::new()
        .route("/", get(routes::light::home_page))
        .route("/fragment/status", get(routes::light::status_fragment))
        .route("/light/power", post(routes::light::toggle_power))
        .route("/light/state", post(routes::light::set_state))
        .route("/metrics", get(routes::light::metrics_endpoint))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to {addr}"));

    tracing::info!("elgato-rs listening on http://{addr}, controlling {light_host}:{light_port}");

    axum::serve(listener, app).await.expect("Server error");
}

async fn poll_light(state: AppState) {
    let mut was_reachable: Option<bool> = None;
    loop {
        match state.client.get_state().await {
            Ok(s) => {
                state.mark_reachable(s).await;
                if was_reachable != Some(true) {
                    tracing::info!("Light is reachable");
                }
                was_reachable = Some(true);
            }
            Err(e) => {
                state.mark_unreachable().await;
                if was_reachable != Some(false) {
                    tracing::warn!("Light is unreachable: {e}");
                }
                was_reachable = Some(false);
            }
        }
        tokio::time::sleep(POLL_INTERVAL).await;
    }
}
