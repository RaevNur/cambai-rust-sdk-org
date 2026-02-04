pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum OrchestratorPipelineResultExceptionReason {
    String(String),

    ExceptionReasons(ExceptionReasons),
}

impl OrchestratorPipelineResultExceptionReason {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_exceptionreasons(&self) -> bool {
        matches!(self, Self::ExceptionReasons(_))
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_exceptionreasons(&self) -> Option<&ExceptionReasons> {
        match self {
            Self::ExceptionReasons(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_exceptionreasons(self) -> Option<ExceptionReasons> {
        match self {
            Self::ExceptionReasons(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for OrchestratorPipelineResultExceptionReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::ExceptionReasons(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
