pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetProbeStreamIn {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passphrase: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<Option<String>>,
}
