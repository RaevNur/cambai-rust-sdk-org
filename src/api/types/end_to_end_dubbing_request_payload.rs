pub use crate::prelude::*;

/// Request type for API operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EndToEndDubbingRequestPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<Option<i64>>,
    pub video_url: String,
    pub source_language: Languages,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_language: Option<Option<Languages>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_languages: Option<Option<Vec<Languages>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_audio_tracks: Option<Option<Vec<i64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_output_as_an_audio_track: Option<Option<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chosen_dictionaries: Option<Option<Vec<i64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_optimization: Option<Option<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
