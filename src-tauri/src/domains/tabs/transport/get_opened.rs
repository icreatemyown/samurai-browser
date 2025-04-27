use crate::domains::{BrowserState, tabs::repo};

#[tauri::command]
pub async fn command_tabs_get_opened(state: BrowserState<'_>) -> Result<Vec<repo::Tab>, usize> {
    let tabs = state.tabs.get_opened().await;
    Ok(tabs)
}
