pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AudioOutputType {
    #[serde(rename = "flac")]
    Flac,
    #[serde(rename = "wav")]
    Wav,
    #[serde(rename = "mp3")]
    Mp3,
    #[serde(rename = "aac")]
    Aac,
    #[serde(rename = "m4a")]
    M4A,
}
impl fmt::Display for AudioOutputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Flac => "flac",
            Self::Wav => "wav",
            Self::Mp3 => "mp3",
            Self::Aac => "aac",
            Self::M4A => "m4a",
        };
        write!(f, "{}", s)
    }
}
