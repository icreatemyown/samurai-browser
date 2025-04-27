use tauri::{Window, WindowEvent};

pub fn on_window_event(window: &Window, event: &WindowEvent) {
    // Get a handle to the app so we can get the global state.
    // let app_handle = window.app_handle();
    // if let WindowEvent::Resized(size) = event {
    //     for ww in window.webviews() {
    //         let _ = ww.clone();
    //         ww.eval("history.back()").expect("Unable to go back");
    //         ww.set_size(*size).expect("Unable to resize");
    //     }
    // };
}
