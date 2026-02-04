pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AddTargetLanguageOut {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<Option<String>>,
}
