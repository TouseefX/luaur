use crate::records::require_node::RequireNode;
use crate::type_aliases::module_name_file_resolver::ModuleName;
use alloc::boxed::Box;

#[repr(C)]
pub struct RequireSuggester {
    pub vtable: RequireSuggesterVtable,
}

#[derive(Clone, Copy)]
pub struct RequireSuggesterVtable {
    pub get_node:
        unsafe fn(*const RequireSuggester, name: &ModuleName) -> Option<Box<dyn RequireNode>>,
}

impl RequireSuggester {
    pub unsafe fn get_node(
        this: *const RequireSuggester,
        name: &ModuleName,
    ) -> Option<Box<dyn RequireNode>> {
        ((*this).vtable.get_node)(this, name)
    }
}

impl core::fmt::Debug for RequireSuggester {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RequireSuggester").finish()
    }
}
