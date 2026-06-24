#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Set {
    /// which registers are in the set that the allocator manages (initialized at construction)
    pub base: u32,

    /// which subset of initial set is free
    pub free: u32,

    /// which subset of initial set is allocated as temporary
    pub temp: u32,

    /// which instruction is defining which register (for spilling); only valid if not free and not temp
    pub defs: [u32; 32],
}

impl Default for Set {
    fn default() -> Self {
        Self {
            base: 0,
            free: 0,
            temp: 0,
            defs: [0; 32],
        }
    }
}
