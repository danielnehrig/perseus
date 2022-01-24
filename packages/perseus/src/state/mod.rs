mod freeze;
mod global_state;
mod page_state_store;
mod rx_state;

pub use freeze::{FrozenApp, PageThawPrefs, ThawPrefs};
pub use global_state::{GlobalState, GlobalStateCreator};
pub use page_state_store::PageStateStore;
pub use rx_state::{AnyFreeze, Freeze, MakeRx, MakeUnrx};

#[cfg(feature = "idb-freezing")]
mod freeze_idb;
#[cfg(feature = "idb-freezing")]
pub use freeze_idb::*; // TODO Be specific here
