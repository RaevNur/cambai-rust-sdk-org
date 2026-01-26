pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConfigStreamPipeline {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demixing: Option<DemixingOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmenting: Option<SegmentingOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcribing: Option<TranscribingOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translating: Option<TranslatingOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoicing: Option<RevoicingOption>,
}
