pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TaskStatus {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "ERROR")]
    Error,
}
impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Success => "SUCCESS",
            Self::Pending => "PENDING",
            Self::Error => "ERROR",
        };
        write!(f, "{}", s)
    }
}
