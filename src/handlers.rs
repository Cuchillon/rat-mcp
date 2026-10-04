use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{Implementation, ServerCapabilities, ServerInfo},
    tool,
    tool_handler,
    tool_router, ErrorData as McpError, ServerHandler,
};

use crate::model::{NavigateParams, SessionParams};
use crate::store::SessionStore;

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
    async fn start_session(
        &self,
    ) -> Result<rmcp::Json<SessionParams>, McpError> {
        match self.store.start_session().await {
            Ok(uuid) => Ok(rmcp::Json(SessionParams { session_id: uuid })),
            Err(error) => Err(McpError::internal_error(
                format!("Session start failed with '{}'", error.to_string()),
                None,
            )),
        }
    }

    #[tool(name = "end_session", description = "Ends a session by its UUID (sessionId).")]
    async fn end_session(
        &self,
        Parameters(params): Parameters<SessionParams>,
    ) -> Result<(), McpError> {
        match self.store.end_session(params.session_id).await {
            Ok(true) => Ok(()),
            Ok(false) => Err(McpError::invalid_params(
                format!("No session with id '{}'", params.session_id),
                None,
            )),
            Err(error) => Err(McpError::internal_error(
                format!("Session end failed with '{error}'"),
                None,
            )),
        }
    }

    #[tool(name = "navigate", description = "Navigates to a page by its URL (url).")]
    async fn navigate(
        &self,
        Parameters(params): Parameters<NavigateParams>,
    ) -> Result<(), McpError> {
        let session = match self.store.get_session(params.session_id).await {
            Some(session) => session,
            None => return Err(McpError::invalid_params(
                format!("No session with id '{}'", params.session_id),
                None,
            )),
        };

        let mut session_guard = session.write().await;

        match session_guard.browser.new_page().await {
            Ok(page) => {
                match page.goto(&params.url, None).await {
                    Ok(_) => {
                        session_guard.current_page = Some(page);
                        Ok(())
                    },
                    Err(error) => Err(McpError::internal_error(error.to_string(), None)),
                }
            },
            Err(error) => Err(McpError::internal_error(error.to_string(), None)),
        }
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for BrowserToolServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions(
                "MCP server providing browser operations.",
            )
            .with_server_info(Implementation::new("rat-mcp", "0.1.0"))
    }
}
