pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OutputFormat {
    #[serde(rename = "wav")]
    Wav,
    #[serde(rename = "flac")]
    Flac,
    #[serde(rename = "adts")]
    Adts,
    #[serde(rename = "mp3")]
    Mp3,
    #[serde(rename = "pcm_s16le")]
    PcmS16Le,
    #[serde(rename = "pcm_s16be")]
    PcmS16Be,
    #[serde(rename = "pcm_s32be")]
    PcmS32Be,
    #[serde(rename = "pcm_s32le")]
    PcmS32Le,
    #[serde(rename = "pcm_f32le")]
    PcmF32Le,
    #[serde(rename = "pcm_f32be")]
    PcmF32Be,
}
impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Wav => "wav",
            Self::Flac => "flac",
            Self::Adts => "adts",
            Self::Mp3 => "mp3",
            Self::PcmS16Le => "pcm_s16le",
            Self::PcmS16Be => "pcm_s16be",
            Self::PcmS32Be => "pcm_s32be",
            Self::PcmS32Le => "pcm_s32le",
            Self::PcmF32Le => "pcm_f32le",
            Self::PcmF32Be => "pcm_f32be",
        };
        write!(f, "{}", s)
    }
}
