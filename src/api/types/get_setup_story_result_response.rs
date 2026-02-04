pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetSetupStoryResultResponse {
    pub run_id: i64,
    pub story_details: StoryDetails,
}
