pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateStoryRequest {
    pub file: Vec<u8>,
    pub source_language: Languages,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub narrator_voice_id: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chosen_dictionaries: Option<Option<Vec<i64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
impl CreateStoryRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        form = form.part(
            "file",
            reqwest::multipart::Part::bytes(self.file.clone())
                .file_name("file")
                .mime_str("application/octet-stream")
                .unwrap(),
        );

        if let Ok(json_str) = serde_json::to_string(&self.source_language) {
            form = form.text("source_language", json_str);
        }

        if let Some(ref value) = self.title {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("title", json_str);
            }
        }

        if let Some(ref value) = self.description {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("description", json_str);
            }
        }

        if let Some(ref value) = self.narrator_voice_id {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("narrator_voice_id", json_str);
            }
        }

        if let Some(ref value) = self.folder_id {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("folder_id", json_str);
            }
        }

        if let Some(ref value) = self.chosen_dictionaries {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("chosen_dictionaries", json_str);
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
