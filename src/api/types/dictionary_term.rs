pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DictionaryTerm {
    pub id: i64,
    pub term_translations: Vec<TermTranslationOutput>,
}
