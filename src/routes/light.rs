use axum::Form;
use axum::extract::State;
use maud::Markup;
use serde::Deserialize;

use crate::elgato::LightState;
use crate::error::AppError;
use crate::metrics;
use crate::state::AppState;
use crate::templates::home;

pub async fn home_page(State(state): State<AppState>) -> Markup {
    let status = *state.status.read().await;
    home::home_page(&status)
}

pub async fn status_fragment(State(state): State<AppState>) -> Markup {
    let status = *state.status.read().await;
    home::status_badge(&status)
}

pub async fn metrics_endpoint(State(state): State<AppState>) -> String {
    let status = *state.status.read().await;
    metrics::render(&status, &state.light_host)
}

/// Cached state if we have it, otherwise a live read from the device.
async fn current_state(state: &AppState) -> Result<LightState, AppError> {
    if let Some(s) = state.status.read().await.state {
        return Ok(s);
    }
    fetch_live(state).await
}

async fn fetch_live(state: &AppState) -> Result<LightState, AppError> {
    match state.client.get_state().await {
        Ok(s) => {
            state.mark_reachable(s).await;
            Ok(s)
        }
        Err(e) => {
            state.mark_unreachable().await;
            Err(e.into())
        }
    }
}

async fn apply(state: &AppState, desired: LightState) -> Result<Markup, AppError> {
    match state.client.set_state(desired).await {
        Ok(applied) => {
            state.mark_reachable(applied).await;
            Ok(home::controls_for(applied))
        }
        Err(e) => {
            state.mark_unreachable().await;
            Err(e.into())
        }
    }
}

pub async fn toggle_power(State(state): State<AppState>) -> Result<Markup, AppError> {
    let mut desired = current_state(&state).await?;
    desired.on = if desired.is_on() { 0 } else { 1 };
    apply(&state, desired).await
}

#[derive(Deserialize)]
pub struct StateForm {
    brightness: u8,
    temperature: u16,
}

pub async fn set_state(
    State(state): State<AppState>,
    Form(form): Form<StateForm>,
) -> Result<Markup, AppError> {
    let current = current_state(&state).await?;
    let desired = LightState {
        on: current.on,
        brightness: form.brightness,
        temperature: form.temperature,
    };
    apply(&state, desired).await
}
