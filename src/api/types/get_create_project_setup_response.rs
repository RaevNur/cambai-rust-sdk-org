pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetCreateProjectSetupResponse {
    pub run_id: i64,
    pub project_details: ProjectDetails,
}
