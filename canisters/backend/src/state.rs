use std::cell::RefCell;

use config::{init_stable_config, Config, StableConfig};
use ic_stable_structures::{memory_manager::MemoryManager, DefaultMemoryImpl};
use queue::ScheduledState;
use utxo_manager::UtxoManager;

pub mod config;
pub mod launch_manager;
pub mod queue;
pub mod user_manager;
pub mod utxo_manager;

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    pub static CONFIG: RefCell<StableConfig> = RefCell::new(init_stable_config());
    pub static UTXO_MANAGER: RefCell<UtxoManager> = RefCell::default();
    pub static SCHEDULED_STATE: RefCell<ScheduledState> = RefCell::default();
}

// helper functions

pub fn read_memory_manager<F, R>(f: F) -> R
where
    F: FnOnce(&MemoryManager<DefaultMemoryImpl>) -> R,
{
    MEMORY_MANAGER.with_borrow(|manager| f(manager))
}

pub fn read_config<F, R>(f: F) -> R
where
    F: FnOnce(&Config) -> R,
{
    CONFIG.with_borrow(|config| f(config.get()))
}

pub fn write_config<F, R>(f: F) -> R
where
    F: FnOnce(&mut StableConfig) -> R,
{
    CONFIG.with_borrow_mut(|config| f(config))
}

pub fn read_utxo_manager<F, R>(f: F) -> R
where
    F: FnOnce(&UtxoManager) -> R,
{
    UTXO_MANAGER.with_borrow(|manager| f(manager))
}

pub fn write_utxo_manager<F, R>(f: F) -> R
where
    F: FnOnce(&mut UtxoManager) -> R,
{
    UTXO_MANAGER.with_borrow_mut(|manager| f(manager))
}

pub fn read_scheduled_state<F, R>(f: F) -> R
where
    F: FnOnce(&ScheduledState) -> R,
{
    SCHEDULED_STATE.with_borrow(|state| f(state))
}

pub fn write_scheduled_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut ScheduledState) -> R,
{
    SCHEDULED_STATE.with_borrow_mut(|state| f(state))
}
