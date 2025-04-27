use std::collections::HashMap;

use itertools::Itertools;
use serde::Serialize;
use uuid::Uuid;

use super::model::tab::Tab;

#[derive(Debug, thiserror::Error, Serialize)]
#[serde(tag = "kind", content = "src")]
pub enum TabError {
    #[error("Tab {0} doesn't exist")]
    InvalidTab(Uuid),
}

#[derive(Default)]
pub struct TabRepo {
    opened_tabs: HashMap<Uuid, Tab>,
    active_tab: Option<Uuid>,
}
impl TabRepo {
    pub fn get_active_tab(&self) -> Option<Tab> {
        self.active_tab
            .and_then(|index| self.opened_tabs.get(&index).cloned())
    }
    pub fn get_opened_tabs(&self) -> Vec<Tab> {
        self.opened_tabs.values().cloned().collect_vec()
    }
    pub fn set_active_tab(&mut self, id: Uuid) -> Result<Tab, TabError> {
        if !self.opened_tabs.contains_key(&id) {
            return Err(TabError::InvalidTab(id));
        }
        self.active_tab.replace(id);
        self.opened_tabs
            .get(&id)
            .cloned()
            .ok_or(TabError::InvalidTab(id))
    }
    pub fn add_tab(&mut self, tab: Tab) -> Uuid {
        let id = tab.label;
        self.opened_tabs.insert(id, tab);
        id
    }
}
