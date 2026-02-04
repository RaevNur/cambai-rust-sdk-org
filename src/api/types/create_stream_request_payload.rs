pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateStreamRequestPayload {
    /// The name of the stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// The description of the stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// The initial delay in seconds before starting the stream creation process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_delay: Option<i64>,
    /// The maximum duration in minutes for the stream creation process before timing out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_mins: Option<Option<i64>>,
    /// List of voice identifiers to be used in the stream.
    pub voices: Vec<i64>,
    /// List of dictionary identifiers to be used in the stream.
    pub dictionaries: Vec<i64>,
    /// The shared configuration for the streaming pipeline.
    pub config: ConfigStream,
    /// The source stream configuration details.
    pub source_stream: SourceStream,
    /// List of target stream configurations.
    pub target_streams: Vec<TargetStream>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Option<DateTime<FixedOffset>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Option<DateTime<FixedOffset>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<Option<String>>,
}
