#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct RegisterLink {
    pub reg: u8,
    pub version: u32,
}

impl Default for RegisterLink {
    fn default() -> Self {
        Self { reg: 0, version: 0 }
    }
}

impl luaur_common::records::dense_hash_table::DenseDefault for RegisterLink {
    fn dense_default() -> Self {
        Self::default()
    }
}
