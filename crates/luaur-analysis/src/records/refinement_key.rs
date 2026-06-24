use alloc::string::String;

/// `DefId` is a `NotNull<const Def>`, which in Rust is represented as a non-null raw pointer to a `Def`.
pub type DefId = *const core::ffi::c_void;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefinementKey {
    pub(crate) parent: *const RefinementKey,
    pub(crate) def: DefId,
    pub(crate) propName: Option<String>,
}

impl Default for RefinementKey {
    fn default() -> Self {
        Self {
            parent: core::ptr::null(),
            def: core::ptr::null(),
            propName: None,
        }
    }
}

#[allow(non_snake_case)]
impl RefinementKey {
    pub fn parent(&self) -> *const RefinementKey {
        self.parent
    }

    pub fn def(&self) -> DefId {
        self.def
    }

    pub fn propName(&self) -> Option<&String> {
        self.propName.as_ref()
    }
}
