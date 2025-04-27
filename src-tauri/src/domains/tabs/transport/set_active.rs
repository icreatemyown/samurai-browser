use tauri::AppHandle;
use uuid::Uuid;

use crate::domains::BrowserState;

#[tauri::command]
pub async fn command_tabs_set_active(
    state: BrowserState<'_>,
    app: AppHandle,
    id: Uuid,
) -> Result<(), String> {
    state
        .tabs
        .set_active_tab(app, id)
        .await
        .inspect_err(|err| println!("{err}"))
        .map_err(|err| format!("{err:?}"))?;
    Ok(())
}
