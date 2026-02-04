pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateDictionaryFromFileRequest {
    pub dictionary_file: Vec<u8>,
    pub dictionary_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dictionary_description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
impl CreateDictionaryFromFileRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        form = form.part(
            "dictionary_file",
            reqwest::multipart::Part::bytes(self.dictionary_file.clone())
                .file_name("dictionary_file")
                .mime_str("application/octet-stream")
                .unwrap(),
        );

        if let Ok(json_str) = serde_json::to_string(&self.dictionary_name) {
            form = form.text("dictionary_name", json_str);
        }

        if let Some(ref value) = self.dictionary_description {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("dictionary_description", json_str);
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
