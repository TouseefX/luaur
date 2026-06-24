use crate::records::recursion_limit_exception::RecursionLimitException;
use crate::records::source_module::SourceModule;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::module_ptr_module::ModulePtr;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use luaur_ast::enums::mode::Mode;

impl TypeChecker {
    pub fn check_source_module_mode_optional_scope_ptr(
        &mut self,
        module: &SourceModule,
        mode: Mode,
        environment_scope: Option<ScopePtr>,
    ) -> ModulePtr {
        match self.check_without_recursion_check(module, mode, environment_scope) {
            module_ptr => module_ptr,
        }
    }
}
