#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockOrdering {
    pub depth: u32,
    pub preOrder: u32,
    pub postOrder: u32,
    pub visited: bool,
}

impl Default for BlockOrdering {
    fn default() -> Self {
        Self {
            depth: 0,
            preOrder: !0u32,
            postOrder: !0u32,
            visited: false,
        }
    }
}

#[allow(non_upper_case_globals)]
impl BlockOrdering {
    pub const preOrder: u32 = !0u32;
    pub const postOrder: u32 = !0u32;
}
