use crate::records::refinement_key::RefinementKey;
use crate::records::refinement_key_arena::RefinementKeyArena;
use crate::type_aliases::def_id_refinement::DefId;
use alloc::string::String;

impl RefinementKeyArena {
    pub fn node(
        &mut self,
        parent: *const RefinementKey,
        def: DefId,
        prop_name: &String,
    ) -> *const RefinementKey {
        self.allocator.allocate(RefinementKey {
            parent,
            def: def.as_ptr() as *const core::ffi::c_void,
            propName: Some(prop_name.clone()),
        })
    }
}
