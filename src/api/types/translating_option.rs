pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TranslatingOption {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "best_model")]
    BestModel,
    #[serde(rename = "fast_model")]
    FastModel,
}
impl fmt::Display for TranslatingOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::None => "none",
            Self::BestModel => "best_model",
            Self::FastModel => "fast_model",
        };
        write!(f, "{}", s)
    }
}
