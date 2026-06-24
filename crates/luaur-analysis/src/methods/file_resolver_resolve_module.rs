use crate::records::file_resolver::FileResolver;
use crate::records::module_info::ModuleInfo;
use crate::records::type_check_limits::TypeCheckLimits;
use luaur_ast::records::ast_expr::AstExpr;

impl FileResolver {
    pub fn resolve_module_impl(
        &self,
        _context: *const ModuleInfo,
        _expr: *mut AstExpr,
        _limits: &TypeCheckLimits,
    ) -> Option<ModuleInfo> {
        None
    }
}
