pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ValidationError {
    pub loc: Vec<ValidationErrorLocItem>,
    pub msg: String,
    pub r#type: String,
}
