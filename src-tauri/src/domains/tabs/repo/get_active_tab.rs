use async_trait::async_trait;

use super::TabRepoImpl;

#[async_trait]
pub trait GetActiveTab {
    fn get_active_tab(&self) -> Option<super::Tab>;
}

#[async_trait]
impl GetActiveTab for TabRepoImpl {
    fn get_active_tab(&self) -> Option<super::Tab> {
        self.opened_tabs.get(self.active_tab).cloned()
    }
}
