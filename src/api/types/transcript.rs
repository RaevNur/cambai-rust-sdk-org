pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transcript {
    pub start: f64,
    pub end: f64,
    pub text: String,
    pub speaker: String,
}
