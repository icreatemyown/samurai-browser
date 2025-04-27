use tauri::AppHandle;
use url::Url;

use crate::domains::{BrowserState, tabs::repo};

#[tauri::command]
pub async fn command_tabs_add(
    state: BrowserState<'_>,
    app: AppHandle,
    url: Url,
) -> Result<repo::Tab, String> {
    let new_tab = state
        .tabs
        .open_new_tab(app, url)
        .await
        .map_err(|err| format!("{err}"))?;
    Ok(new_tab)
}
