use crate::records::frontend_module_resolver::FrontendModuleResolver;
use crate::records::module_info::ModuleInfo;
use crate::type_aliases::module_name_file_resolver::ModuleName;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_node::AstNode;

impl FrontendModuleResolver {
    pub fn resolve_module_info(
        &self,
        current_module_name: &ModuleName,
        path_expr: &AstExpr,
    ) -> Option<ModuleInfo> {
        if self.frontend.is_null() {
            return None;
        }

        let frontend = unsafe { &*self.frontend };
        let trace = frontend.require_trace.get(current_module_name)?;
        let key = path_expr as *const AstExpr as *mut AstNode;

        trace.exprs.find(&key).cloned()
    }
}
