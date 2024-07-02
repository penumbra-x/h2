use http::HeaderMap;

use crate::frame::PseudoType;

const X_CLIENT_PROFILE: &str = "x-client-profile";

/// This is the default user agent profile used by the library.
/// It can be overridden by setting the `x-client-profile` header.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum AgentProfile {
    Chrome,
    Firefox,
    Safari,
    Edge,
    OkHttp,
}

impl AgentProfile {
    /// To header (key, value)
    pub fn to_header(&self) -> (&'static str, &'static str) {
        (
            X_CLIENT_PROFILE,
            match self {
                AgentProfile::Chrome => "chrome",
                AgentProfile::Firefox => "firefox",
                AgentProfile::Safari => "safari",
                AgentProfile::Edge => "edge",
                AgentProfile::OkHttp => "okhttp",
            },
        )
    }
}

/// Default to Chrome
impl Default for AgentProfile {
    fn default() -> Self {
        AgentProfile::Chrome
    }
}

/// Convert a string to a profile
impl From<&str> for AgentProfile {
    fn from(s: &str) -> Self {
        match s {
            "chrome" => AgentProfile::Chrome,
            "firefox" => AgentProfile::Firefox,
            "safari" => AgentProfile::Safari,
            "edge" => AgentProfile::Edge,
            "okhttp" => AgentProfile::OkHttp,
            _ => AgentProfile::Chrome,
        }
    }
}

impl Into<[PseudoType; 4]> for AgentProfile {
    fn into(self) -> [PseudoType; 4] {
        match self {
            AgentProfile::Chrome | AgentProfile::Edge => [
                PseudoType::Method,
                PseudoType::Authority,
                PseudoType::Scheme,
                PseudoType::Path,
            ],
            AgentProfile::OkHttp => [
                PseudoType::Method,
                PseudoType::Path,
                PseudoType::Authority,
                PseudoType::Scheme,
            ],
            AgentProfile::Safari => [
                PseudoType::Method,
                PseudoType::Scheme,
                PseudoType::Path,
                PseudoType::Authority,
            ],
            AgentProfile::Firefox => [
                PseudoType::Method,
                PseudoType::Path,
                PseudoType::Authority,
                PseudoType::Scheme,
            ],
        }
    }
}

/// Convert HeaderMap to profile, will remove the header
impl From<&mut HeaderMap> for AgentProfile {
    fn from(headers: &mut HeaderMap) -> Self {
        let profile = headers
            .get(X_CLIENT_PROFILE)
            .and_then(|v| v.to_str().ok().map(Self::from))
            .unwrap_or(Self::Chrome);

        // Create a new HeaderMap to preserve the order of headers while removing X_CLIENT_PROFILE.
        let mut new_headers = HeaderMap::new();
        for (name, value) in headers.iter() {
            // Insert all headers into the new map except for X_CLIENT_PROFILE.
            if !name.as_str().eq(X_CLIENT_PROFILE) {
                new_headers.insert(name, value.clone());
            }
        }

        // Replace the original headers with the new set, effectively removing X_CLIENT_PROFILE.
        *headers = new_headers;

        profile
    }
}
