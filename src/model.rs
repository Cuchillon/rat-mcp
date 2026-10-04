use playwright_rs::{Browser, Page};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct Session {
    pub browser: Browser,
    pub current_page: Option<Page>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SessionParams {
    #[serde(rename = "sessionId")]
    pub session_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NavigateParams {
    #[serde(rename = "sessionId")]
    pub session_id: Uuid,
    pub url: String,
}
