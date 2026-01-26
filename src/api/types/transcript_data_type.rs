pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TranscriptDataType {
    #[serde(rename = "raw_data")]
    RawData,
    #[serde(rename = "file")]
    File,
}
impl fmt::Display for TranscriptDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::RawData => "raw_data",
            Self::File => "file",
        };
        write!(f, "{}", s)
    }
}
