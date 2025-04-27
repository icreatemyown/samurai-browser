pub mod create_new_tab;
pub mod get_active_tab;
pub mod get_opened_tabs;
pub mod set_active_tab;

use create_new_tab::CreateNewTab;
use get_active_tab::GetActiveTab;
use get_opened_tabs::GetOpenedTabs;
use serde::Serialize;
use set_active_tab::SetActiveTab;
use url::Url;
use uuid::Uuid;

use crate::service_trait;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tab {
    pub id: Uuid,
    pub url: Url,
    pub custom_title: Option<String>,
}

#[derive(Default)]
pub struct TabRepoImpl {
    opened_tabs: Vec<Tab>,
    active_tab: usize,
}

service_trait!(TabRepo: GetOpenedTabs, GetActiveTab, CreateNewTab, SetActiveTab);
