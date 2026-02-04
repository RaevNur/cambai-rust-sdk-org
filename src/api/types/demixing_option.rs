pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DemixingOption {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "pick_left_channel")]
    PickLeftChannel,
    #[serde(rename = "pick_right_channel")]
    PickRightChannel,
    #[serde(rename = "pick_center_channel")]
    PickCenterChannel,
    #[serde(rename = "best_model")]
    BestModel,
    #[serde(rename = "fast_model")]
    FastModel,
}
impl fmt::Display for DemixingOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::None => "none",
            Self::PickLeftChannel => "pick_left_channel",
            Self::PickRightChannel => "pick_right_channel",
            Self::PickCenterChannel => "pick_center_channel",
            Self::BestModel => "best_model",
            Self::FastModel => "fast_model",
        };
        write!(f, "{}", s)
    }
}
