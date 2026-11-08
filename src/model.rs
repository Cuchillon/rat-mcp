use playwright_rs::{Browser, Page, DEFAULT_TIMEOUT_MS};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn default_headless() -> bool {
    false
}

fn default_timeout() -> f64 {
    DEFAULT_TIMEOUT_MS
}

#[derive(Debug)]
pub struct Session {
    pub browser: Browser,
    pub page: Page,
    pub meta: SessionMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SessionMeta {
    #[serde(default = "default_headless")]
    pub headless: bool,
    #[serde(rename = "globalWaitTimeout", default = "default_timeout")]
    pub global_wait_timeout: f64,
    #[serde(rename = "globalElementTimeout", default = "default_timeout")]
    pub global_element_timeout: f64,
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
    #[serde(rename = "waitTimeout")]
    pub wait_timeout: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ElementParams {
    #[serde(rename = "sessionId")]
    pub session_id: Uuid,
    pub locator: String,
    #[serde(rename = "elementTimeout")]
    pub element_timeout: Option<f64>,
}
