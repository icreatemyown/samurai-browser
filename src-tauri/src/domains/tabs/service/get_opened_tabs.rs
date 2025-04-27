use async_trait::async_trait;

use crate::domains::tabs::repo::{self};

use super::TabServiceImpl;

#[async_trait]
pub trait GetOpenedTabs {
    async fn get_opened(&self) -> Vec<repo::Tab>;
}

#[async_trait]
impl GetOpenedTabs for TabServiceImpl {
    async fn get_opened(&self) -> Vec<repo::Tab> {
        let repo = self.repo.lock().await;
        repo.get_opened_tabs().await
    }
}
