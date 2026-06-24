use crate::records::file_resolver::{
    file_resolver_get_environment_for_module_default,
    file_resolver_get_human_readable_module_name_default, file_resolver_resolve_module_default,
    FileResolver, FileResolverVtable,
};
use crate::records::require_suggester::RequireSuggester;
use crate::records::source_code::SourceCode;
use crate::type_aliases::module_name_file_resolver::ModuleName;
use alloc::sync::Arc;

impl FileResolver {
    pub fn file_resolver_shared_ptr_require_suggester(
        require_suggester: Arc<RequireSuggester>,
    ) -> Self {
        Self {
            vtable: FileResolverVtable {
                read_source: |_, _| -> Option<SourceCode> { panic!("read_source is pure virtual") },
                resolve_module: file_resolver_resolve_module_default,
                get_human_readable_module_name:
                    file_resolver_get_human_readable_module_name_default,
                get_environment_for_module: file_resolver_get_environment_for_module_default,
            },
            require_suggester: Some(require_suggester),
        }
    }
}
