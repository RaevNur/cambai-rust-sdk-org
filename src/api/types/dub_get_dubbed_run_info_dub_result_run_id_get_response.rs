pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GetDubbedRunInfoDubResultRunIDGetResponse {
    DubbingResult(DubbingResult),

    Map1(HashMap<String, DubbingResult>),
}

impl GetDubbedRunInfoDubResultRunIDGetResponse {
    pub fn is_dubbingresult(&self) -> bool {
        matches!(self, Self::DubbingResult(_))
    }

    pub fn is_map1(&self) -> bool {
        matches!(self, Self::Map1(_))
    }

    pub fn as_dubbingresult(&self) -> Option<&DubbingResult> {
        match self {
            Self::DubbingResult(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_dubbingresult(self) -> Option<DubbingResult> {
        match self {
            Self::DubbingResult(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_map1(&self) -> Option<&HashMap<String, DubbingResult>> {
        match self {
            Self::Map1(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_map1(self) -> Option<HashMap<String, DubbingResult>> {
        match self {
            Self::Map1(value) => Some(value),
            _ => None,
        }
    }
}
