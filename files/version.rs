//! Static build info for Jetbrains Code With Me.

pub const BUILD_VERSION: &str  = "v3.5.9";
pub const BUILD_CHANNEL: &str  = "stable";
pub const BUILD_COMMIT: &str   = "aa7363197a65";
pub const BUILD_TIMESTAMP: u64 = 1794954227;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Jetbrains Code With Me", BUILD_VERSION, BUILD_CHANNEL)
}
