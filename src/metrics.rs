use crate::elgato::mireds_to_kelvin;
use crate::state::LightStatus;

pub fn render(status: &LightStatus, host: &str) -> String {
    let mut out = String::new();

    out.push_str("# HELP elgato_light_reachable Whether the light responded to the most recent poll (1 = reachable).\n");
    out.push_str("# TYPE elgato_light_reachable gauge\n");
    out.push_str(&format!(
        "elgato_light_reachable{{host=\"{host}\"}} {}\n",
        status.reachable as u8
    ));

    // State gauges reflect the last successful read; they persist while the
    // light is unreachable so dashboards keep the last-known values, with
    // elgato_light_reachable distinguishing stale from live.
    if let Some(s) = status.state {
        out.push_str("# HELP elgato_light_on Whether the light is on (1 = on).\n");
        out.push_str("# TYPE elgato_light_on gauge\n");
        out.push_str(&format!("elgato_light_on{{host=\"{host}\"}} {}\n", s.on));

        out.push_str("# HELP elgato_light_brightness_percent Light brightness (3-100).\n");
        out.push_str("# TYPE elgato_light_brightness_percent gauge\n");
        out.push_str(&format!(
            "elgato_light_brightness_percent{{host=\"{host}\"}} {}\n",
            s.brightness
        ));

        out.push_str("# HELP elgato_light_temperature_mireds Color temperature in mireds (143-344).\n");
        out.push_str("# TYPE elgato_light_temperature_mireds gauge\n");
        out.push_str(&format!(
            "elgato_light_temperature_mireds{{host=\"{host}\"}} {}\n",
            s.temperature
        ));

        out.push_str("# HELP elgato_light_temperature_kelvin Approximate color temperature in Kelvin.\n");
        out.push_str("# TYPE elgato_light_temperature_kelvin gauge\n");
        out.push_str(&format!(
            "elgato_light_temperature_kelvin{{host=\"{host}\"}} {}\n",
            mireds_to_kelvin(s.temperature)
        ));
    }

    out
}
