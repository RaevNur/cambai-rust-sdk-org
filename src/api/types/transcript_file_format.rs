pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TranscriptFileFormat {
    #[serde(rename = "srt")]
    Srt,
    #[serde(rename = "vtt")]
    Vtt,
    #[serde(rename = "txt")]
    Txt,
}
impl fmt::Display for TranscriptFileFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Srt => "srt",
            Self::Vtt => "vtt",
            Self::Txt => "txt",
        };
        write!(f, "{}", s)
    }
}
