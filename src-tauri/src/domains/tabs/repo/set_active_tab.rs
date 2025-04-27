use async_trait::async_trait;
use itertools::Itertools;
use uuid::Uuid;

use super::TabRepoImpl;

#[async_trait]
pub trait SetActiveTab {
    fn set_active_tab_by_id(&mut self, id: &Uuid) -> Option<super::Tab>;
}

#[async_trait]
impl SetActiveTab for TabRepoImpl {
    fn set_active_tab_by_id(&mut self, id: &Uuid) -> Option<super::Tab> {
        let existing_tab = self.opened_tabs.iter().find_position(|tab| id == &tab.id);
        if let Some((pos, tab)) = existing_tab {
            self.active_tab = pos;
            Some(tab.clone())
        } else {
            None
        }
    }
}
