mod errors;
mod handlers;
mod model;
mod store;

use rmcp::{transport::stdio, ServiceExt};

use crate::handlers::BrowserToolServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = BrowserToolServer::new().await?;
    let (stdin, stdout) = stdio();
    let running = server.clone().serve((stdin, stdout)).await?;
    let result = running.waiting().await;
    server.close_all().await;
    result?;
    Ok(())
}
