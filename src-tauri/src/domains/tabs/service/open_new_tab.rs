use async_trait::async_trait;
use eyre::{Context, ContextCompat};
use tauri::{AppHandle, Emitter, Manager, WebviewBuilder, WindowEvent};
use url::Url;

use crate::domains::tabs::repo;

use super::TabServiceImpl;

#[async_trait]
pub trait OpenNewTab {
    async fn open_new_tab(&self, app: AppHandle, url: Url) -> eyre::Result<repo::Tab>;
}

#[async_trait]
impl OpenNewTab for TabServiceImpl {
    async fn open_new_tab(&self, app: AppHandle, url: Url) -> eyre::Result<repo::Tab> {
        let (new_tab, new_tab_id) = {
            let mut repo = self.repo.lock().await;
            let new_tab_id = repo.create_new_tab(url.clone(), None);
            let new_tab = repo
                .set_active_tab_by_id(&new_tab_id)
                .wrap_err("New tab wasn't created for some reason")?;
            (new_tab, new_tab_id)
        };
        let padding_y = 200;
        let root_window = app
            .get_window("main")
            .wrap_err("Unable to get main window")?;
        let root_window_size = root_window
            .inner_size()
            .wrap_err("Unable to get window size")?;

        {
            let webview_url = tauri::WebviewUrl::External(url);
            let webview_builder = WebviewBuilder::new(new_tab_id, webview_url);
            let webview = root_window
                .add_child(
                    webview_builder,
                    tauri::PhysicalPosition::new(0, padding_y),
                    tauri::PhysicalSize::new(
                        root_window_size.width,
                        root_window_size.height.saturating_sub(padding_y),
                    ),
                )
                .wrap_err("Unable to add new webview")?;
            webview.show().wrap_err("Unable to show webview")?;

            root_window.on_window_event(move |event| {
                if let WindowEvent::Resized(root_window_size) = event {
                    if let Err(err) = webview
                        .set_size(tauri::PhysicalSize::new(
                            root_window_size.width,
                            root_window_size.height.saturating_sub(padding_y),
                        ))
                        .wrap_err("Unable to update webview size")
                    {
                        tracing::error!("{err:#?}");
                    }
                }
            });
            root_window.emit("tabs:list:changed", ())?;
        }

        Ok(new_tab)
    }
}
