pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ExceptionReasons {
    #[serde(rename = "HARMFUL_CONTENT_DETECTED")]
    HarmfulContentDetected,
    #[serde(rename = "VOICE_CONVERSION_ERROR")]
    VoiceConversionError,
    #[serde(rename = "PROCESSING_ERROR")]
    ProcessingError,
    #[serde(rename = "SOURCE_TOO_LONG")]
    SourceTooLong,
    #[serde(rename = "SOURCE_TOO_LARGE")]
    SourceTooLarge,
    #[serde(rename = "SOURCE_TYPE_NOT_SUPPORTED")]
    SourceTypeNotSupported,
    #[serde(rename = "ERROR_DOWNLOADING_SOURCE")]
    ErrorDownloadingSource,
    #[serde(rename = "TOO_MANY_GDRIVE_REQUESTS")]
    TooManyGdriveRequests,
    #[serde(rename = "SOURCE_BLOCKED_IN_REGION")]
    SourceBlockedInRegion,
    #[serde(rename = "SOURCE_IS_AGE_RESTRICTED")]
    SourceIsAgeRestricted,
    #[serde(rename = "SOURCE_NOT_FOUND")]
    SourceNotFound,
    #[serde(rename = "MISMATCHED_SOURCE_CODEC")]
    MismatchedSourceCodec,
    #[serde(rename = "CONTENT_DOES_NOT_MATCH_EXTENSION")]
    ContentDoesNotMatchExtension,
    #[serde(rename = "INVALID_SOURCE_DATA")]
    InvalidSourceData,
    #[serde(rename = "VIDEO_DOES_NOT_HAVE_AUDIO_STREAMS")]
    VideoDoesNotHaveAudioStreams,
    #[serde(rename = "MASTERING_OUT_OF_RANGE")]
    MasteringOutOfRange,
    #[serde(rename = "INVALID_AUDIO_TRACKS_SELECTION")]
    InvalidAudioTracksSelection,
    #[serde(rename = "PAYMENT_REQUIRED")]
    PaymentRequired,
    #[serde(rename = "FORBIDDEN")]
    Forbidden,
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError,
    #[serde(rename = "NONE")]
    None,
}
impl fmt::Display for ExceptionReasons {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::HarmfulContentDetected => "HARMFUL_CONTENT_DETECTED",
            Self::VoiceConversionError => "VOICE_CONVERSION_ERROR",
            Self::ProcessingError => "PROCESSING_ERROR",
            Self::SourceTooLong => "SOURCE_TOO_LONG",
            Self::SourceTooLarge => "SOURCE_TOO_LARGE",
            Self::SourceTypeNotSupported => "SOURCE_TYPE_NOT_SUPPORTED",
            Self::ErrorDownloadingSource => "ERROR_DOWNLOADING_SOURCE",
            Self::TooManyGdriveRequests => "TOO_MANY_GDRIVE_REQUESTS",
            Self::SourceBlockedInRegion => "SOURCE_BLOCKED_IN_REGION",
            Self::SourceIsAgeRestricted => "SOURCE_IS_AGE_RESTRICTED",
            Self::SourceNotFound => "SOURCE_NOT_FOUND",
            Self::MismatchedSourceCodec => "MISMATCHED_SOURCE_CODEC",
            Self::ContentDoesNotMatchExtension => "CONTENT_DOES_NOT_MATCH_EXTENSION",
            Self::InvalidSourceData => "INVALID_SOURCE_DATA",
            Self::VideoDoesNotHaveAudioStreams => "VIDEO_DOES_NOT_HAVE_AUDIO_STREAMS",
            Self::MasteringOutOfRange => "MASTERING_OUT_OF_RANGE",
            Self::InvalidAudioTracksSelection => "INVALID_AUDIO_TRACKS_SELECTION",
            Self::PaymentRequired => "PAYMENT_REQUIRED",
            Self::Forbidden => "FORBIDDEN",
            Self::InternalError => "INTERNAL_ERROR",
            Self::None => "NONE",
        };
        write!(f, "{}", s)
    }
}
