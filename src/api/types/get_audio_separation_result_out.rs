pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetAudioSeparationResultOut {
    pub foreground_audio_url: String,
    pub background_audio_url: String,
}
