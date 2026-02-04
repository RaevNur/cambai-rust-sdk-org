pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetTextToVoiceResultOut {
    pub previews: Vec<String>,
}
