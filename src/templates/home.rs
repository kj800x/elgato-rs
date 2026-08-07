use maud::{Markup, html};

use crate::elgato::{
    LightState, MAX_BRIGHTNESS, MAX_TEMPERATURE, MIN_BRIGHTNESS, MIN_TEMPERATURE, mireds_to_kelvin,
};
use crate::state::LightStatus;
use crate::templates::layout::layout;

pub fn home_page(status: &LightStatus) -> Markup {
    layout(
        "Control",
        html! {
            section .light-card {
                div #status hx-get="/fragment/status" hx-trigger="every 5s" hx-swap="innerHTML" {
                    (status_badge(status))
                }
                (controls(status))
            }
        },
    )
}

pub fn status_badge(status: &LightStatus) -> Markup {
    html! {
        @if status.reachable {
            span .badge.badge-ok { "Connected" }
        } @else {
            span .badge.badge-err { "Unreachable" }
        }
    }
}

pub fn controls(status: &LightStatus) -> Markup {
    let Some(s) = status.state else {
        return html! {
            div #controls .light-controls {
                p .error-message { "No contact with the light yet — check its power cord." }
                a .btn href="/" { "Retry" }
            }
        };
    };
    controls_for(s)
}

pub fn controls_for(s: LightState) -> Markup {
    html! {
        div #controls .light-controls {
            button .power-btn.(if s.is_on() { "on" } else { "off" })
                hx-post="/light/power" hx-target="#controls" hx-swap="outerHTML"
                title=(if s.is_on() { "Turn off" } else { "Turn on" }) {
                (if s.is_on() { "On" } else { "Off" })
            }
            form hx-post="/light/state" hx-target="#controls" hx-swap="outerHTML" hx-trigger="change" {
                div .slider-row {
                    label for="brightness" {
                        "Brightness "
                        span #bval .slider-value { (s.brightness) "%" }
                    }
                    input type="range" id="brightness" name="brightness"
                        min=(MIN_BRIGHTNESS) max=(MAX_BRIGHTNESS) value=(s.brightness)
                        oninput="document.getElementById('bval').textContent = this.value + '%'";
                }
                div .slider-row {
                    label for="temperature" {
                        "Temperature "
                        span #tval .slider-value { (mireds_to_kelvin(s.temperature)) " K" }
                    }
                    input type="range" id="temperature" name="temperature" .temp-slider
                        min=(MIN_TEMPERATURE) max=(MAX_TEMPERATURE) value=(s.temperature)
                        oninput="document.getElementById('tval').textContent = Math.round(1000000 / this.value) + ' K'";
                    div .slider-legend {
                        span { "Cool" }
                        span { "Warm" }
                    }
                }
            }
        }
    }
}
