pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LanguagePydanticModel {
    pub id: i64,
    pub language: String,
    pub short_name: String,
}
