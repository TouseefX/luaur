use crate::records::refinement_key::RefinementKey;
use crate::records::refinement_key_arena::RefinementKeyArena;
use crate::type_aliases::def_id_refinement::DefId;

impl RefinementKeyArena {
    pub fn leaf(&mut self, def: DefId) -> *const RefinementKey {
        self.allocator.allocate(RefinementKey {
            parent: core::ptr::null(),
            def: def.as_ptr() as *const core::ffi::c_void,
            propName: None,
        })
    }
}
