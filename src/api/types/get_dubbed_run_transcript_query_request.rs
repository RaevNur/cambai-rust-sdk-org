pub use crate::prelude::*;

/// Query parameters for getDubbedRunTranscript
///
/// Request type for the GetDubbedRunTranscriptQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDubbedRunTranscriptQueryRequest {
    /// Format to use for the transcription. Either `srt`, `vtt` or `txt`. Defaults to `txt`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_type: Option<TranscriptFileFormat>,
    /// Data type for the transcription being returned. Returns the raw data of the transcription or a presigned url for the file that holds the transcription data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<TranscriptDataType>,
}
