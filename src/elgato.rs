use serde::{Deserialize, Serialize};
use std::time::Duration;

pub const MIN_BRIGHTNESS: u8 = 3;
pub const MAX_BRIGHTNESS: u8 = 100;
/// Mireds; 143 ≈ 7000K (coolest), 344 ≈ 2900K (warmest)
pub const MIN_TEMPERATURE: u16 = 143;
pub const MAX_TEMPERATURE: u16 = 344;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LightState {
    pub on: u8,
    pub brightness: u8,
    pub temperature: u16,
}

impl LightState {
    pub fn is_on(&self) -> bool {
        self.on != 0
    }

    pub fn clamped(self) -> Self {
        Self {
            on: self.on.min(1),
            brightness: self.brightness.clamp(MIN_BRIGHTNESS, MAX_BRIGHTNESS),
            temperature: self.temperature.clamp(MIN_TEMPERATURE, MAX_TEMPERATURE),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LightsPayload {
    number_of_lights: u8,
    lights: Vec<LightState>,
}

pub fn mireds_to_kelvin(mireds: u16) -> u32 {
    1_000_000 / mireds.max(1) as u32
}

#[derive(Clone)]
pub struct ElgatoClient {
    http: reqwest::Client,
    base: String,
}

impl ElgatoClient {
    pub fn new(host: &str, port: u16) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(3))
            .build()
            .expect("Failed to build HTTP client");
        Self {
            http,
            base: format!("http://{host}:{port}"),
        }
    }

    pub async fn get_state(&self) -> Result<LightState, reqwest::Error> {
        let payload: LightsPayload = self
            .http
            .get(format!("{}/elgato/lights", self.base))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(first_light(payload))
    }

    /// The device echoes the resulting state in the PUT response.
    pub async fn set_state(&self, state: LightState) -> Result<LightState, reqwest::Error> {
        let payload = LightsPayload {
            number_of_lights: 1,
            lights: vec![state.clamped()],
        };
        let resp: LightsPayload = self
            .http
            .put(format!("{}/elgato/lights", self.base))
            .json(&payload)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(first_light(resp))
    }
}

fn first_light(payload: LightsPayload) -> LightState {
    payload.lights.into_iter().next().unwrap_or(LightState {
        on: 0,
        brightness: MIN_BRIGHTNESS,
        temperature: MIN_TEMPERATURE,
    })
}
