use serde::Serialize;
use uuid::Uuid;

use super::tab_addr::TabAddr;

#[derive(Clone, Serialize)]
pub struct Tab {
    pub label: Uuid,
    pub addr: TabAddr,
}

impl From<TabAddr> for Tab {
    fn from(addr: TabAddr) -> Self {
        Self {
            addr,
            label: Uuid::new_v4(),
        }
    }
}
