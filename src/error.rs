use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use maud::html;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Unreachable(reqwest::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Unreachable(e) => write!(f, "Light unreachable: {}", e),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Unreachable(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::warn!("{}", self);
        let body = html! {
            div #controls .light-controls {
                p .error-message {
                    "The light didn't respond — check its power cord."
                }
                a .btn href="/" { "Retry" }
            }
        };
        (StatusCode::BAD_GATEWAY, body).into_response()
    }
}
