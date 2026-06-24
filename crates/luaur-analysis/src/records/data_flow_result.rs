use crate::records::refinement_key::RefinementKey;

/// `DefId` is a `NotNull<const Def>`, which in Rust is represented as a non-null raw pointer to a `Def`.
/// Since `Def` is an opaque or yet-to-be-translated struct in this context, we use `*const c_void` or a placeholder.
/// Based on the provided `Def.h` fragment, `DefId` is a pointer to a `Def`.
pub type DefId = *const core::ffi::c_void;

#[derive(Debug, Clone, Copy)]
pub struct DataFlowResult {
    pub def: DefId,
    pub parent: *const RefinementKey,
}

impl Default for DataFlowResult {
    fn default() -> Self {
        Self {
            def: core::ptr::null(),
            parent: core::ptr::null(),
        }
    }
}
