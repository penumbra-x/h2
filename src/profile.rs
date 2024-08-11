use crate::frame::{PseudoType, StreamDependency, StreamId};

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
    /// To pseudo types
    pub(crate) fn to_pseudo(&self) -> [PseudoType; 4] {
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

    #[allow(dead_code)]
    pub(crate) fn to_stream_dependency(&self) -> StreamDependency {
        match self {
            AgentProfile::Chrome
            | AgentProfile::Edge
            | AgentProfile::OkHttp
            | AgentProfile::Firefox => StreamDependency::new(StreamId::zero(), 255, true),
            AgentProfile::Safari => StreamDependency::new(StreamId::zero(), 254, false),
        }
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
