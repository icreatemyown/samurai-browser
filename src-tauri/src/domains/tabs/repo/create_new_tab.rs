use async_trait::async_trait;
use url::Url;
use uuid::Uuid;

use super::{Tab, TabRepoImpl};

#[async_trait]
pub trait CreateNewTab {
    fn create_new_tab(&mut self, url: Url, title: Option<String>) -> Uuid;
}

#[async_trait]
impl CreateNewTab for TabRepoImpl {
    fn create_new_tab(&mut self, url: Url, custom_title: Option<String>) -> Uuid {
        let id = Uuid::new_v4();
        let tab = Tab {
            id,
            url,
            custom_title,
        };
        self.opened_tabs.push(tab);
        id
    }
}
