#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct RegisterSet {
    pub regs: [u64; 4],

    // If variadic sequence is active, we track register from which it starts
    pub vararg_seq: bool,
    pub vararg_start: u8,
}

impl Default for RegisterSet {
    fn default() -> Self {
        Self {
            regs: [0; 4],
            vararg_seq: false,
            vararg_start: 0,
        }
    }
}
