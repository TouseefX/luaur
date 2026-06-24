use crate::methods::naive_file_resolver_resolve_module::naive_file_resolver_resolve_module_vtable;
use luaur_analysis::records::null_file_resolver::NullFileResolver;

#[derive(Debug)]
#[repr(C)]
pub struct NaiveFileResolver {
    pub base: NullFileResolver,
}

impl Default for NaiveFileResolver {
    fn default() -> Self {
        let mut base = NullFileResolver::new();
        base.base.vtable.resolve_module = naive_file_resolver_resolve_module_vtable;

        Self { base }
    }
}
