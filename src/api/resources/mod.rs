//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **AudioSeparation**
//! - **Dub**
//! - **Folders**
//! - **Languages**
//! - **Story**
//! - **TranslatedStory**
//! - **TextToAudio**
//! - **TextToVoice**
//! - **TextToSpeech**
//! - **Translation**
//! - **Transcription**
//! - **TranslatedTts**
//! - **Streaming**
//! - **VoiceCloning**
//! - **Dictionaries**
//! - **ProjectSetup**
//! - **DeprecatedStreaming**

use crate::{ApiError, ClientConfig};

pub mod audio_separation;
pub mod deprecated_streaming;
pub mod dictionaries;
pub mod dub;
pub mod folders;
pub mod languages;
pub mod project_setup;
pub mod story;
pub mod streaming;
pub mod text_to_audio;
pub mod text_to_speech;
pub mod text_to_voice;
pub mod transcription;
pub mod translated_story;
pub mod translated_tts;
pub mod translation;
pub mod voice_cloning;
pub struct APIClient {
    pub config: ClientConfig,
    pub audio_separation: AudioSeparationClient,
    pub dub: DubClient,
    pub folders: FoldersClient,
    pub languages: LanguagesClient,
    pub story: StoryClient,
    pub translated_story: TranslatedStoryClient,
    pub text_to_audio: TextToAudioClient,
    pub text_to_voice: TextToVoiceClient,
    pub text_to_speech: TextToSpeechClient,
    pub translation: TranslationClient,
    pub transcription: TranscriptionClient,
    pub translated_tts: TranslatedTtsClient,
    pub streaming: StreamingClient,
    pub voice_cloning: VoiceCloningClient,
    pub dictionaries: DictionariesClient,
    pub project_setup: ProjectSetupClient,
    pub deprecated_streaming: DeprecatedStreamingClient,
}

impl APIClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            audio_separation: AudioSeparationClient::new(config.clone())?,
            dub: DubClient::new(config.clone())?,
            folders: FoldersClient::new(config.clone())?,
            languages: LanguagesClient::new(config.clone())?,
            story: StoryClient::new(config.clone())?,
            translated_story: TranslatedStoryClient::new(config.clone())?,
            text_to_audio: TextToAudioClient::new(config.clone())?,
            text_to_voice: TextToVoiceClient::new(config.clone())?,
            text_to_speech: TextToSpeechClient::new(config.clone())?,
            translation: TranslationClient::new(config.clone())?,
            transcription: TranscriptionClient::new(config.clone())?,
            translated_tts: TranslatedTtsClient::new(config.clone())?,
            streaming: StreamingClient::new(config.clone())?,
            voice_cloning: VoiceCloningClient::new(config.clone())?,
            dictionaries: DictionariesClient::new(config.clone())?,
            project_setup: ProjectSetupClient::new(config.clone())?,
            deprecated_streaming: DeprecatedStreamingClient::new(config.clone())?,
        })
    }
}

pub use audio_separation::AudioSeparationClient;
pub use deprecated_streaming::DeprecatedStreamingClient;
pub use dictionaries::DictionariesClient;
pub use dub::DubClient;
pub use folders::FoldersClient;
pub use languages::LanguagesClient;
pub use project_setup::ProjectSetupClient;
pub use story::StoryClient;
pub use streaming::StreamingClient;
pub use text_to_audio::TextToAudioClient;
pub use text_to_speech::TextToSpeechClient;
pub use text_to_voice::TextToVoiceClient;
pub use transcription::TranscriptionClient;
pub use translated_story::TranslatedStoryClient;
pub use translated_tts::TranslatedTtsClient;
pub use translation::TranslationClient;
pub use voice_cloning::VoiceCloningClient;
