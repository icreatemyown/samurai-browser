use std::sync::Mutex;

use tauri::{Emitter, Manager, State, WebviewBuilder, Window, WindowEvent};
use uuid::Uuid;

use super::{
    model::{tab::Tab, tab_addr::TabAddr},
    repo::{TabError, TabRepo},
};

#[tauri::command]
pub async fn tabs_add(
    state: State<'_, Mutex<TabRepo>>,
    window: tauri::Window,
    url: &str,
) -> Result<Tab, TabError> {
    let addr = TabAddr::from(url);
    let tab = Tab::from(addr);

    let padding_y = 200;

    {
        let webview_url = (&tab.addr).into();
        let webview_builder = WebviewBuilder::new(tab.label, webview_url);
        let window_size = window.inner_size().expect("Unable to get window size");
        let webview = window
            .add_child(
                webview_builder,
                tauri::PhysicalPosition::new(0, padding_y),
                tauri::PhysicalSize::new(window_size.width, window_size.height - padding_y),
            )
            .expect("Unable to add new tab");
        webview.show().expect("Unable to show webview");
        window.on_window_event(move |event| {
            if let WindowEvent::Resized(size) = event {
                webview
                    .set_size(tauri::PhysicalSize::new(
                        size.width,
                        size.height - padding_y,
                    ))
                    .expect("Unable to update webview size");
            }
        });
    }

    window
        .emit("tabs:list:changed", ())
        .expect("Unable to emit event");

    let mut tab_service = state.lock().expect("Unable to get state");
    let added_index = tab_service.add_tab(tab);

    tab_service.set_active_tab(added_index)
}

#[tauri::command]
pub fn tabs_get(state: State<Mutex<TabRepo>>) -> Vec<Tab> {
    let tab_service = state.lock().expect("Unable to get state");
    tab_service.get_opened_tabs()
}

#[tauri::command]
pub async fn tabs_set_active(
    state: State<'_, Mutex<TabRepo>>,
    window: Window,
    id: Uuid,
) -> Result<Tab, TabError> {
    let mut tab_service = state.lock().expect("Unable to get state");
    if let Some(ww) = tab_service
        .get_active_tab()
        .map(|t| t.label.to_string())
        .and_then(|l| window.get_webview(&l))
    {
        ww.hide().expect("Unable to hide");
    };
    if let Some(ww) = window.get_webview(&id.to_string()) {
        ww.show().expect("Unable to show");
    }
    tab_service.set_active_tab(id)
}
