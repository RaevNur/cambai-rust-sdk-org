use crate::api::*;
use crate::{ApiError, ByteStream, RequestOptions};
use async_trait::async_trait;
use serde_json::json;
use reqwest::{Client, Method};

#[async_trait]
pub trait TtsProvider: Send + Sync {
    async fn tts(
        &self,
        request: &CreateStreamTtsRequestPayload,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError>;
}

pub struct DefaultProvider {
    client: crate::api::resources::APIClient,
}

impl DefaultProvider {
    pub fn new(client: crate::api::resources::APIClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl TtsProvider for DefaultProvider {
    async fn tts(
        &self,
        request: &CreateStreamTtsRequestPayload,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.client.text_to_speech.tts(request, options).await
    }
}

pub struct BasetenProvider {
    api_key: String,
    url: String,
    http_client: Client,
}

impl BasetenProvider {
    pub fn new(api_key: String, url: Option<String>) -> Self {
        Self {
            api_key,
            url: url.unwrap_or_else(|| "https://model-5qeryx53.api.baseten.co/environments/production/predict".to_string()),
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl TtsProvider for BasetenProvider {
    async fn tts(
        &self,
        request: &CreateStreamTtsRequestPayload,
        _options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        let lang_str = request.language.to_string();

        let payload = json!({
            "text": request.text,
            "stream": true,
            "output_format": "mp3",
            "language": lang_str,
            "reference_audio": "DUMMY_BASE64", // Placeholder
            "audio_ref": "DUMMY_BASE64",
            "reference_language": "en-us",
            "apply_ner_nlp": false,
        });

        let response = self.http_client
            .request(Method::POST, &self.url)
            .header("Authorization", format!("Api-Key {}", self.api_key))
            .json(&payload)
            .send()
            .await
            .map_err(ApiError::Network)?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            return Err(ApiError::from_response(status, Some(&text)));
        }

        Ok(ByteStream::new(response))
    }
}
