use async_trait::async_trait;
use eyre::{Context, ContextCompat};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use super::TabServiceImpl;

#[async_trait]
pub trait SetActiveTab {
    async fn set_active_tab(&self, app: AppHandle, id: Uuid) -> eyre::Result<()>;
}

#[async_trait]
impl SetActiveTab for TabServiceImpl {
    async fn set_active_tab(&self, app: AppHandle, id: Uuid) -> eyre::Result<()> {
        let active_tab = self
            .repo
            .lock()
            .await
            .set_active_tab_by_id(&id)
            .wrap_err("Tab does not exist")?;
        let window = app
            .get_window("main")
            .wrap_err("Unable to get root window")?;
        for ww in window.webviews() {
            if active_tab.id.to_string() == ww.label() {
                ww.show().wrap_err("Unable to open webview")?;
            } else {
                ww.hide().wrap_err("Unable to hide webview")?;
            }
        }

        Ok(())
    }
}
