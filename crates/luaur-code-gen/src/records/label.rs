#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Label {
    pub id: u32,
    pub location: u32,
}

impl Default for Label {
    fn default() -> Self {
        Self {
            id: 0,
            location: !0u32,
        }
    }
}

#[allow(non_upper_case_globals)]
impl Label {
    pub const id: u32 = 0;
    pub const location: u32 = !0u32;
}
