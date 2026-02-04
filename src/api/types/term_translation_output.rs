pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TermTranslationOutput {
    pub language: i64,
    pub term_text: String,
}
