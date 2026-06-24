use crate::records::module_info::ModuleInfo;
use crate::records::require_suggester::RequireSuggester;
use crate::records::source_code::SourceCode;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::type_aliases::module_name_file_resolver::ModuleName;
use alloc::string::String;
use alloc::sync::Arc;
use luaur_ast::records::ast_expr::AstExpr;

#[repr(C)]
pub struct FileResolver {
    pub vtable: FileResolverVtable,
    pub require_suggester: Option<Arc<RequireSuggester>>,
}

#[derive(Clone, Copy)]
pub struct FileResolverVtable {
    pub read_source: unsafe fn(*mut FileResolver, name: &ModuleName) -> Option<SourceCode>,
    pub resolve_module: unsafe fn(
        *mut FileResolver,
        context: *const ModuleInfo,
        expr: *mut AstExpr,
        limits: &TypeCheckLimits,
    ) -> Option<ModuleInfo>,
    pub get_human_readable_module_name: unsafe fn(*const FileResolver, name: &ModuleName) -> String,
    pub get_environment_for_module:
        unsafe fn(*const FileResolver, name: &ModuleName) -> Option<String>,
}

pub(crate) unsafe fn file_resolver_resolve_module_default(
    _this: *mut FileResolver,
    _context: *const ModuleInfo,
    _expr: *mut AstExpr,
    _limits: &TypeCheckLimits,
) -> Option<ModuleInfo> {
    None
}

pub(crate) unsafe fn file_resolver_get_human_readable_module_name_default(
    _this: *const FileResolver,
    name: &ModuleName,
) -> String {
    name.clone()
}

pub(crate) unsafe fn file_resolver_get_environment_for_module_default(
    _this: *const FileResolver,
    _name: &ModuleName,
) -> Option<String> {
    None
}

impl FileResolver {
    pub unsafe fn read_source(this: *mut FileResolver, name: &ModuleName) -> Option<SourceCode> {
        ((*this).vtable.read_source)(this, name)
    }

    pub unsafe fn resolve_module(
        this: *mut FileResolver,
        context: *const ModuleInfo,
        expr: *mut AstExpr,
        limits: &TypeCheckLimits,
    ) -> Option<ModuleInfo> {
        ((*this).vtable.resolve_module)(this, context, expr, limits)
    }

    pub unsafe fn get_human_readable_module_name(
        this: *const FileResolver,
        name: &ModuleName,
    ) -> String {
        ((*this).vtable.get_human_readable_module_name)(this, name)
    }

    pub unsafe fn get_environment_for_module(
        this: *const FileResolver,
        name: &ModuleName,
    ) -> Option<String> {
        ((*this).vtable.get_environment_for_module)(this, name)
    }
}

impl core::fmt::Debug for FileResolver {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FileResolver")
            .field("require_suggester", &self.require_suggester)
            .finish()
    }
}
