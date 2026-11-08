use crate::errors::{internal_error, internal_error_with_message, invalid_params};
use crate::model::{ElementParams, NavigateParams, Session, SessionParams};
use crate::store::SessionStore;
use playwright_rs::WaitForOptions;
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{Implementation, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router, ErrorData as McpError, ServerHandler,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct BrowserToolServer {
    store: SessionStore,
    tool_router: ToolRouter<Self>,
}

impl BrowserToolServer {
    pub async fn new() -> Result<Self, playwright_rs::Error> {
        Ok(Self {
            store: SessionStore::with_playwright().await?,
            tool_router: Self::tool_router(),
        })
    }

    pub async fn close_all(&self) {
        self.store.close_all().await;
    }
}

#[tool_router]
impl BrowserToolServer {
    #[tool(
        name = "start_session",
        description = "Starts a session. Returns JSON with fields: sessionId."
    )]
    async fn start_session(&self) -> Result<rmcp::Json<SessionParams>, McpError> {
        match self.store.start_session().await {
            Ok(uuid) => Ok(rmcp::Json(SessionParams { session_id: uuid })),
            Err(error) => Err(internal_error_with_message(format!(
                "Session start failed with '{error}'"
            ))),
        }
    }

    #[tool(
        name = "end_session",
        description = "Ends a session by its UUID (sessionId)."
    )]
    async fn end_session(
        &self,
        Parameters(params): Parameters<SessionParams>,
    ) -> Result<(), McpError> {
        match self.store.end_session(params.session_id).await {
            Ok(true) => Ok(()),
            Ok(false) => Err(invalid_params(format!(
                "No session with id '{}'",
                params.session_id
            ))),
            Err(error) => Err(internal_error_with_message(format!(
                "Session end failed with '{error}'"
            ))),
        }
    }

    #[tool(
        name = "navigate",
        description = "Navigates to a page by its URL (url)."
    )]
    async fn navigate(
        &self,
        Parameters(params): Parameters<NavigateParams>,
    ) -> Result<(), McpError> {
        let session = self.get_session(params.session_id).await?;
        let mut session_guard = session.write().await;

        if session_guard.page.is_closed() {
            session_guard.page = session_guard
                .browser
                .new_page()
                .await
                .map_err(internal_error)?;
        }

        session_guard
            .page
            .goto(&params.url, None)
            .await
            .map_err(internal_error)?;
        session_guard
            .page
            .locator(params.wait_for_loaded)
            .wait_for(WaitForOptions::builder().timeout(4000.0).build())
            .await
            .map_err(internal_error)?;
        Ok(())
    }

    #[tool(
        name = "click",
        description = "Clicks an element by its Locator (locator)."
    )]
    async fn click(&self, Parameters(params): Parameters<ElementParams>) -> Result<(), McpError> {
        let session = self.get_session(params.session_id).await?;
        let session_guard = session.write().await;

        session_guard
            .page
            .locator(params.locator)
            .click(None)
            .await
            .map_err(internal_error)?;
        Ok(())
    }

    async fn get_session(&self, id: Uuid) -> Result<Arc<RwLock<Session>>, McpError> {
        self.store
            .get_session(id)
            .await
            .ok_or_else(|| invalid_params(format!("No session with id '{}'", id)))
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for BrowserToolServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions("MCP server providing browser operations.")
            .with_server_info(Implementation::new("rat-mcp", "0.1.0"))
    }
}
