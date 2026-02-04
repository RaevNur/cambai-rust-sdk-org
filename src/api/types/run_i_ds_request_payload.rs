pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RunIDsRequestPayload {
    pub run_ids: Vec<i64>,
}
