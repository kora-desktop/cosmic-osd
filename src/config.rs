use cosmic_config::{Config, ConfigGet};

const AUDIO_CONFIG: &str = "com.system76.CosmicAudio";
const AMPLIFICATION_SINK: &str = "amplification_sink";
const AMPLIFICATION_SOURCE: &str = "amplification_source";

pub fn amplification_sink() -> bool {
    Config::new(AUDIO_CONFIG, 1)
        .ok()
        .and_then(|config| config.get::<bool>(AMPLIFICATION_SINK).ok())
        .unwrap_or(true)
}

pub fn amplification_source() -> bool {
    Config::new(AUDIO_CONFIG, 1)
        .ok()
        .and_then(|config| config.get::<bool>(AMPLIFICATION_SOURCE).ok())
        .unwrap_or(false)
}

const OSD_CONFIG: &str = "com.system76.CosmicOsd";
const VALUE_INDICATORS: &str = "value_indicators";

/// Whether volume and brightness changes show an indicator. A system that
/// shows them its own way, or not at all, sets this false in its default
/// config (`/usr/share/cosmic/com.system76.CosmicOsd/v1/value_indicators`).
pub fn value_indicators() -> bool {
    Config::new(OSD_CONFIG, 1)
        .ok()
        .and_then(|config| config.get::<bool>(VALUE_INDICATORS).ok())
        .unwrap_or(true)
}

const POLKIT_AGENT: &str = "polkit_agent";

/// Whether this process is the session's polkit agent. A system whose own
/// shell answers polkit sets this false in its default config
/// (`/usr/share/cosmic/com.system76.CosmicOsd/v1/polkit_agent`), so the two
/// never race to register for the session.
pub fn polkit_agent() -> bool {
    Config::new(OSD_CONFIG, 1)
        .ok()
        .and_then(|config| config.get::<bool>(POLKIT_AGENT).ok())
        .unwrap_or(true)
}
