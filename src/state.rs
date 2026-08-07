use std::sync::Arc;
use tokio::sync::RwLock;

use crate::elgato::{ElgatoClient, LightState};

#[derive(Debug, Clone, Copy, Default)]
pub struct LightStatus {
    pub reachable: bool,
    /// Last state successfully read from the device.
    pub state: Option<LightState>,
}

#[derive(Clone)]
pub struct AppState {
    pub client: ElgatoClient,
    pub status: Arc<RwLock<LightStatus>>,
    pub light_host: String,
}

impl AppState {
    pub async fn mark_reachable(&self, state: LightState) {
        *self.status.write().await = LightStatus {
            reachable: true,
            state: Some(state),
        };
    }

    pub async fn mark_unreachable(&self) {
        self.status.write().await.reachable = false;
    }
}
