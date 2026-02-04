pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VideoOutputTypeWithoutAvi {
    #[serde(rename = "mkv")]
    Mkv,
    #[serde(rename = "mp4")]
    Mp4,
    #[serde(rename = "mov")]
    Mov,
    #[serde(rename = "mxf")]
    Mxf,
}
impl fmt::Display for VideoOutputTypeWithoutAvi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Mkv => "mkv",
            Self::Mp4 => "mp4",
            Self::Mov => "mov",
            Self::Mxf => "mxf",
        };
        write!(f, "{}", s)
    }
}
