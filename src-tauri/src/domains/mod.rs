use std::sync::Arc;

use tabs::service::{TabService, TabServiceImpl};
use tauri::State;

mod custom_macro;
pub mod tabs;
pub mod windows;

type BrowserState<'a> = State<'a, BrowserApp>;

pub struct BrowserApp {
    pub tabs: Arc<dyn TabService>,
}
impl BrowserApp {
    pub fn new() -> Self {
        let tabs = TabServiceImpl::new();
        let tabs = Arc::new(tabs);

        Self { tabs }
    }
}
