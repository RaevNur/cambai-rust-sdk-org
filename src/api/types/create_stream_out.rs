pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateStreamOut {
    pub stream_id: i64,
    pub stream_url_for_languages: Vec<StreamURLForLanguages>,
    pub task_id: String,
}
