pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TextToAudioType {
    #[serde(rename = "sound")]
    Sound,
    #[serde(rename = "music")]
    Music,
}
impl fmt::Display for TextToAudioType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Sound => "sound",
            Self::Music => "music",
        };
        write!(f, "{}", s)
    }
}
