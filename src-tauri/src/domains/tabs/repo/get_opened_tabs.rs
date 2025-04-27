use async_trait::async_trait;

use super::TabRepoImpl;

#[async_trait]
pub trait GetOpenedTabs {
    async fn get_opened_tabs(&self) -> Vec<super::Tab>;
}

#[async_trait]
impl GetOpenedTabs for TabRepoImpl {
    async fn get_opened_tabs(&self) -> Vec<super::Tab> {
        self.opened_tabs.clone()
    }
}
