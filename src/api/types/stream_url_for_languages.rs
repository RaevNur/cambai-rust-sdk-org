pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StreamURLForLanguages {
    pub languages: Vec<Languages>,
    pub url: String,
}
