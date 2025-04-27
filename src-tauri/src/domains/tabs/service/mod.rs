mod get_opened_tabs;
mod open_new_tab;
mod set_active_tab;

use std::sync::Arc;

use get_opened_tabs::GetOpenedTabs;
use open_new_tab::OpenNewTab;
use set_active_tab::SetActiveTab;
use tauri::async_runtime::Mutex;

use crate::service_trait;

use super::repo::{TabRepo, TabRepoImpl};

service_trait!(TabService: OpenNewTab, GetOpenedTabs, SetActiveTab);

pub struct TabServiceImpl {
    repo: Arc<Mutex<dyn TabRepo>>,
}
impl TabServiceImpl {
    pub fn new() -> Self {
        let repo = TabRepoImpl::default();
        let repo = Arc::new(Mutex::new(repo));
        Self { repo }
    }
}
