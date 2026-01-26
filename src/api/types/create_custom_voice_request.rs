pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateCustomVoiceRequest {
    pub voice_name: String,
    pub gender: Gender,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_voice_to_market_place: Option<Option<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Option<Languages>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhance_audio: Option<Option<bool>>,
    pub file: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
impl CreateCustomVoiceRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        form = form.part(
            "file",
            reqwest::multipart::Part::bytes(self.file.clone())
                .file_name("file")
                .mime_str("application/octet-stream")
                .unwrap(),
        );

        if let Ok(json_str) = serde_json::to_string(&self.voice_name) {
            form = form.text("voice_name", json_str);
        }

        if let Ok(json_str) = serde_json::to_string(&self.gender) {
            form = form.text("gender", json_str);
        }

        if let Some(ref value) = self.description {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("description", json_str);
            }
        }

        if let Some(ref value) = self.publish_voice_to_market_place {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("publish_voice_to_market_place", json_str);
            }
        }

        if let Some(ref value) = self.age {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("age", json_str);
            }
        }

        if let Some(ref value) = self.language {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("language", json_str);
            }
        }

        if let Some(ref value) = self.enhance_audio {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("enhance_audio", json_str);
            }
        }

        if let Some(ref value) = self.run_id {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("run_id", json_str);
            }
        }

        form
    }
}
