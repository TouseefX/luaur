use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl crate::records::tarjan::Tarjan {
    pub fn set_dirty(&mut self, index: i32, d: bool) {
        let index_usize = index as usize;
        LUAU_ASSERT!(index_usize < self.nodes.len());
        self.nodes[index_usize].dirty = d;
    }
}
