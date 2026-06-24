use crate::enums::control_flow::ControlFlow;
use crate::records::generic_error::GenericError;
use crate::records::type_checker::TypeChecker;
use crate::records::type_error::TypeError;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use luaur_ast::records::ast_stat_type_function::AstStatTypeFunction;

impl TypeChecker {
    pub fn check_scope_ptr_ast_stat_type_function(
        &mut self,
        _scope: &ScopePtr,
        typefunction: &AstStatTypeFunction,
    ) -> ControlFlow {
        self.report_error_location_type_error_data(
            &typefunction.base.base.location,
            TypeErrorData::GenericError(GenericError::new(
                "This syntax is not supported".to_string(),
            )),
        );

        ControlFlow::None
    }
}
