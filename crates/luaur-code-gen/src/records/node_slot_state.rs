#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct NodeSlotState {
    pub pointer: u32,
    pub knownToNotBeNil: bool,
}

#[allow(non_upper_case_globals)]
impl NodeSlotState {
    pub const pointer: u32 = 0;
    pub const knownToNotBeNil: bool = false;
}
