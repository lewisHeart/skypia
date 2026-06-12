use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BannerInfo {
    pub text: String,
    pub action_label: String,
    pub link: String,
    pub icon: String,
    pub image_url: Option<String>,
}
