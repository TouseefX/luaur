use crate::records::module_info::ModuleInfo;
use crate::records::null_module_resolver::NullModuleResolver;
use crate::type_aliases::module_name_type_fwd::ModuleName;
use luaur_ast::records::ast_expr::AstExpr;

impl NullModuleResolver {
    pub fn resolve_module_info(
        &mut self,
        _current_module_name: &ModuleName,
        _path_expr: &AstExpr,
    ) -> Option<ModuleInfo> {
        None
    }
}
