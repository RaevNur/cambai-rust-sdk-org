//! API client and types for the FastAPI
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints
//! - [`types`] - Request, response, and model types

pub mod resources;
pub mod types;

pub use resources::{
    APIClient, AudioSeparationClient, DeprecatedStreamingClient, DictionariesClient, DubClient,
    FoldersClient, LanguagesClient, ProjectSetupClient, StoryClient, StreamingClient,
    TextToAudioClient, TextToSpeechClient, TextToVoiceClient, TranscriptionClient,
    TranslatedStoryClient, TranslatedTtsClient, TranslationClient, VoiceCloningClient,
};
pub use types::*;
