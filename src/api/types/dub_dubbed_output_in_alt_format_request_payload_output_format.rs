pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum DubbedOutputInAltFormatRequestPayloadOutputFormat {
    AudioOutputType(AudioOutputType),

    VideoOutputTypeWithoutAvi(VideoOutputTypeWithoutAvi),
}

impl DubbedOutputInAltFormatRequestPayloadOutputFormat {
    pub fn is_audiooutputtype(&self) -> bool {
        matches!(self, Self::AudioOutputType(_))
    }

    pub fn is_videooutputtypewithoutavi(&self) -> bool {
        matches!(self, Self::VideoOutputTypeWithoutAvi(_))
    }

    pub fn as_audiooutputtype(&self) -> Option<&AudioOutputType> {
        match self {
            Self::AudioOutputType(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_audiooutputtype(self) -> Option<AudioOutputType> {
        match self {
            Self::AudioOutputType(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_videooutputtypewithoutavi(&self) -> Option<&VideoOutputTypeWithoutAvi> {
        match self {
            Self::VideoOutputTypeWithoutAvi(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_videooutputtypewithoutavi(self) -> Option<VideoOutputTypeWithoutAvi> {
        match self {
            Self::VideoOutputTypeWithoutAvi(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for DubbedOutputInAltFormatRequestPayloadOutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AudioOutputType(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::VideoOutputTypeWithoutAvi(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
