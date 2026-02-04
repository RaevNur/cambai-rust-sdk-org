pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TermTranslationInput {
    pub translation: String,
    pub language: Languages,
}
