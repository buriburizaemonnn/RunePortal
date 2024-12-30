use ic_stable_structures::{
    memory_manager::{MemoryId, VirtualMemory},
    DefaultMemoryImpl,
};

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

pub enum MemoryIds {
    Config,
    Runic,
    Bitcoin,
}

impl From<MemoryIds> for MemoryId {
    fn from(ids: MemoryIds) -> MemoryId {
        let id = match ids {
            MemoryIds::Config => 1,
            MemoryIds::Runic => 2,
            MemoryIds::Bitcoin => 3,
        };
        MemoryId::new(id)
    }
}
