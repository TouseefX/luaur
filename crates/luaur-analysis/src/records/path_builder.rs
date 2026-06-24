//! @interface-stub
use crate::records::path::Path;
use crate::type_aliases::component::Component;
use crate::type_aliases::type_pack_id::TypePackId;

#[derive(Debug, Clone)]
pub struct PathBuilder {
    pub(crate) components: alloc::vec::Vec<Component>,
}

impl PathBuilder {
    pub fn new() -> Self {
        Self {
            components: alloc::vec::Vec::new(),
        }
    }
}

impl Default for PathBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// All builder methods are implemented faithfully in `methods/path_builder_*.rs`
// (single-word ops via `PathBuilder*` traits; `prop`/`read_prop`/`write_prop`
// as inherent impls), mirroring `TypePath.cpp:174-272`.
