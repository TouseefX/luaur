//! `DemoFileResolver::resolveModule` (`CLI/src/Web.cpp:27-33`).
//!
//! ```cpp
//! std::optional<Luau::ModuleInfo> resolveModule(const Luau::ModuleInfo* context, Luau::AstExpr* expr, const Luau::TypeCheckLimits& limits) override
//! {
//!     if (Luau::AstExprGlobal* g = expr->as<Luau::AstExprGlobal>())
//!         return Luau::ModuleInfo{g->name.value};
//!
//!     return std::nullopt;
//! }
//! ```

use crate::records::demo_file_resolver::DemoFileResolver;
use luaur_analysis::records::module_info::ModuleInfo;
use luaur_analysis::records::type_check_limits::TypeCheckLimits;
use luaur_analysis::type_aliases::module_name_file_resolver::ModuleName;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;
use core::ffi::CStr;

impl DemoFileResolver {
    pub fn resolve_module(
        &self,
        _context: *const ModuleInfo,
        expr: *mut AstExpr,
        _limits: &TypeCheckLimits,
    ) -> Option<ModuleInfo> {
        // expr->as<Luau::AstExprGlobal>()
        let g = unsafe { ast_node_as::<AstExprGlobal>(expr as *mut AstNode) };
        if g.is_null() {
            return None;
        }

        // ModuleInfo{g->name.value} — g->name.value is the interned C string
        // naming the global; its contents become the (string) ModuleName.
        let value = unsafe { (*g).name.value };
        let name: ModuleName = if value.is_null() {
            ModuleName::default()
        } else {
            unsafe { CStr::from_ptr(value) }
                .to_string_lossy()
                .into_owned()
        };

        Some(ModuleInfo {
            name,
            optional: false,
        })
    }
}
