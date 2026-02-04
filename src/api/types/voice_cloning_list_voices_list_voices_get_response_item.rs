pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ListVoicesListVoicesGetResponseItem {
    Map0(HashMap<String, serde_json::Value>),

    Voice(Voice),
}

impl ListVoicesListVoicesGetResponseItem {
    pub fn is_map0(&self) -> bool {
        matches!(self, Self::Map0(_))
    }

    pub fn is_voice(&self) -> bool {
        matches!(self, Self::Voice(_))
    }

    pub fn as_map0(&self) -> Option<&HashMap<String, serde_json::Value>> {
        match self {
            Self::Map0(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_map0(self) -> Option<HashMap<String, serde_json::Value>> {
        match self {
            Self::Map0(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_voice(&self) -> Option<&Voice> {
        match self {
            Self::Voice(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_voice(self) -> Option<Voice> {
        match self {
            Self::Voice(value) => Some(value),
            _ => None,
        }
    }
}
