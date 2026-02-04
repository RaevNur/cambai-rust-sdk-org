pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAudioSeparationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_file: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
impl CreateAudioSeparationRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        if let Some(ref file_data) = self.media_file {
            form = form.part(
                "media_file",
                reqwest::multipart::Part::bytes(file_data.clone())
                    .file_name("media_file")
                    .mime_str("application/octet-stream")
                    .unwrap(),
            );
        }

        if let Some(ref value) = self.project_name {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("project_name", json_str);
            }
        }

        if let Some(ref value) = self.project_description {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("project_description", json_str);
            }
        }

        if let Some(ref value) = self.folder_id {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("folder_id", json_str);
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
