//! Port of `DemoFileResolver : Luau::FileResolver` (`CLI/src/Web.cpp:16-46`).
//!
//! The luau.org/demo file resolver. Like the project's `NullFileResolver`, it is
//! a concrete `FileResolver` subclass: `#[repr(C)]` with `base: FileResolver`
//! first so a `*mut FileResolver` (the vtable receiver) can be cast back to
//! `*mut DemoFileResolver` to reach `source`. The four virtuals are wired to the
//! free `demo_file_resolver_*` thunks; each delegates to the inherent methods in
//! `methods/`.
//!
//! C++ member: `std::unordered_map<ModuleName, std::string> source;` — ported as
//! a typed `HashMap<ModuleName, String>` (no untyped JSON).

use luaur_analysis::records::file_resolver::{FileResolver, FileResolverVtable};
use luaur_analysis::records::module_info::ModuleInfo;
use luaur_analysis::records::source_code::SourceCode;
use luaur_analysis::records::type_check_limits::TypeCheckLimits;
use luaur_analysis::type_aliases::module_name_file_resolver::ModuleName;
use luaur_ast::records::ast_expr::AstExpr;
use alloc::string::String;
use std::collections::HashMap;

#[repr(C)]
pub struct DemoFileResolver {
    pub base: FileResolver,
    pub source: HashMap<ModuleName, String>,
}

/// `std::optional<SourceCode> readSource(const ModuleName&)` thunk.
///
/// # Safety
/// `this` must point at the `base` subobject of a live `DemoFileResolver`.
pub(crate) unsafe fn demo_file_resolver_read_source_thunk(
    this: *mut FileResolver,
    name: &ModuleName,
) -> Option<SourceCode> {
    let this = this as *mut DemoFileResolver;
    (*this).read_source(name)
}

/// `std::optional<ModuleInfo> resolveModule(const ModuleInfo*, AstExpr*, const TypeCheckLimits&)` thunk.
///
/// # Safety
/// `this` must point at the `base` subobject of a live `DemoFileResolver`.
pub(crate) unsafe fn demo_file_resolver_resolve_module_thunk(
    this: *mut FileResolver,
    context: *const ModuleInfo,
    expr: *mut AstExpr,
    limits: &TypeCheckLimits,
) -> Option<ModuleInfo> {
    let this = this as *const DemoFileResolver;
    (*this).resolve_module(context, expr, limits)
}

/// `std::string getHumanReadableModuleName(const ModuleName&) const` thunk.
///
/// # Safety
/// `this` must point at the `base` subobject of a live `DemoFileResolver`.
pub(crate) unsafe fn demo_file_resolver_get_human_readable_module_name_thunk(
    this: *const FileResolver,
    name: &ModuleName,
) -> String {
    let this = this as *const DemoFileResolver;
    (*this).get_human_readable_module_name(name)
}

/// `std::optional<std::string> getEnvironmentForModule(const ModuleName&) const` thunk.
///
/// # Safety
/// `this` must point at the `base` subobject of a live `DemoFileResolver`.
pub(crate) unsafe fn demo_file_resolver_get_environment_for_module_thunk(
    this: *const FileResolver,
    name: &ModuleName,
) -> Option<String> {
    let this = this as *const DemoFileResolver;
    (*this).get_environment_for_module(name)
}

impl DemoFileResolver {
    pub fn new() -> Self {
        let vtable = FileResolverVtable {
            read_source: demo_file_resolver_read_source_thunk,
            resolve_module: demo_file_resolver_resolve_module_thunk,
            get_human_readable_module_name: demo_file_resolver_get_human_readable_module_name_thunk,
            get_environment_for_module: demo_file_resolver_get_environment_for_module_thunk,
        };

        DemoFileResolver {
            base: FileResolver {
                vtable,
                require_suggester: None,
            },
            source: HashMap::new(),
        }
    }
}

impl Default for DemoFileResolver {
    fn default() -> Self {
        Self::new()
    }
}
