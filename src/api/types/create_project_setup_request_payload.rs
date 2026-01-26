pub use crate::prelude::*;

/// Request type for API operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateProjectSetupRequestPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<Option<i64>>,
    pub media_url: String,
    pub source_language: Languages,
    pub target_languages: Vec<Languages>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_audio_tracks: Option<Option<Vec<i64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_output_as_an_audio_track: Option<Option<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chosen_dictionaries: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
