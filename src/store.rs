use playwright_rs::{LaunchOptions, Playwright};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::model::Session;

#[derive(Clone)]
pub struct SessionStore {
    playwright: Arc<Playwright>,
    sessions: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Session>>>>>,
}

impl SessionStore {
    pub async fn with_playwright() -> Result<Self, playwright_rs::Error> {
        let playwright = Playwright::launch().await?;
        Ok(Self {
            playwright: Arc::new(playwright),
            sessions: Arc::new(HashMap::new().into()),
        })
    }

    pub async fn start_session(&self) -> Result<Uuid, playwright_rs::Error> {
        let chromium = self.playwright.chromium();
        let browser = chromium
            .launch_with_options(LaunchOptions::new().headless(false))
            .await?;
        let page = match browser.new_page().await {
            Ok(page) => page,
            Err(error) => {
                if let Err(close_error) = browser.close().await {
                    eprintln!("Failed to close browser after page creation error: {close_error}");
                }
                return Err(error);
            }
        };
        let id = Uuid::new_v4();
        self.sessions
            .write()
            .await
            .insert(id, Arc::new(RwLock::new(Session { browser, page })));
        Ok(id)
    }

    pub async fn end_session(&self, id: Uuid) -> bool {
        let Some(session) = self.sessions.write().await.remove(&id) else {
            return false;
        };
        if let Err(error) = session.write().await.browser.close().await {
            eprintln!("Failed to close browser on session end: {error}");
        }
        true
    }

    pub async fn get_session(&self, id: Uuid) -> Option<Arc<RwLock<Session>>> {
        self.sessions.read().await.get(&id).cloned()
    }

    pub async fn close_all(&self) {
        let sessions: Vec<_> = self
            .sessions
            .write()
            .await
            .drain()
            .map(|(_, session)| session)
            .collect();
        for session in sessions {
            let session_guard = session.write().await;
            if let Err(error) = session_guard.browser.close().await {
                eprintln!("Failed to close browser on shutdown: {error}");
            }
        }
    }
}
