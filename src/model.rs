use playwright_rs::{Browser, Page};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct Session {
    pub browser: Browser,
    pub page: Page,
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
    #[serde(rename = "waitForLoaded")]
    pub wait_for_loaded: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ElementParams {
    #[serde(rename = "sessionId")]
    pub session_id: Uuid,
    pub locator: String,
}
