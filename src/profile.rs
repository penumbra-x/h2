use http::HeaderMap;

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
    pub fn to_header(&self) -> (&'static str, String) {
        (X_CLIENT_PROFILE, self.to_string())
    }
}

/// Default to Chrome
impl Default for AgentProfile {
    fn default() -> Self {
        AgentProfile::Chrome
    }
}

/// Convert the profile to a string
impl ToString for AgentProfile {
    fn to_string(&self) -> String {
        match self {
            AgentProfile::Chrome => "chrome",
            AgentProfile::Firefox => "firefox",
            AgentProfile::Safari => "safari",
            AgentProfile::Edge => "edge",
            AgentProfile::OkHttp => "okhttp",
        }
        .to_owned()
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

/// Convert HeaderMap to profile, will remove the header
impl From<&mut HeaderMap> for AgentProfile {
    fn from(headers: &mut HeaderMap) -> Self {
        let profile = match headers.get(X_CLIENT_PROFILE) {
            Some(profile) => match profile.to_str() {
                Ok(v) => Self::from(v),
                Err(_) => Self::Chrome,
            },
            None => Self::Chrome,
        };
        headers.remove(X_CLIENT_PROFILE);
        profile
    }
}
