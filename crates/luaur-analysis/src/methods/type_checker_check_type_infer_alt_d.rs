use crate::enums::control_flow::ControlFlow;
use crate::functions::matches::matches;
use crate::functions::operator_bitor_control_flow::operator_bitor as control_flow_or;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use luaur_ast::records::ast_stat_if::AstStatIf;

impl TypeChecker {
    pub fn check_scope_ptr_ast_stat_if(
        &mut self,
        scope: &ScopePtr,
        statement: &AstStatIf,
    ) -> ControlFlow {
        let result = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
            scope,
            unsafe { &*statement.condition },
            None,
            false,
        );

        let then_scope =
            self.child_scope(scope, unsafe { &(*statement.thenbody).base.base.location });
        self.resolve_predicate_vec_scope_ptr_bool(&result.predicates, &then_scope, true);

        let else_location = if !statement.elsebody.is_null() {
            unsafe { (*statement.elsebody).base.location }
        } else {
            statement.base.base.location
        };
        // `statement.elsebody->location` -> AstStat.base(AstNode).location;
        // `statement.location` -> AstStatIf.base(AstStat).base(AstNode).location.
        let else_scope = self.child_scope(scope, &else_location);
        self.resolve_predicate_vec_scope_ptr_bool(&result.predicates, &else_scope, false);

        let thencf =
            self.check_scope_ptr_ast_stat(&then_scope, unsafe { &(*statement.thenbody).base });
        let mut elsecf = ControlFlow::None;
        if !statement.elsebody.is_null() {
            elsecf = self.check_scope_ptr_ast_stat(&else_scope, unsafe { &*statement.elsebody });
        }

        if thencf != ControlFlow::None && elsecf == ControlFlow::None {
            unsafe {
                let scope_mut =
                    alloc::sync::Arc::as_ptr(scope) as *mut crate::records::scope::Scope;
                (*scope_mut).inherit_refinements(&else_scope);
            }
        } else if thencf == ControlFlow::None && elsecf != ControlFlow::None {
            unsafe {
                let scope_mut =
                    alloc::sync::Arc::as_ptr(scope) as *mut crate::records::scope::Scope;
                (*scope_mut).inherit_refinements(&then_scope);
            }
        }

        if thencf == elsecf {
            thencf
        } else if matches(
            thencf,
            control_flow_or(ControlFlow::Returns, ControlFlow::Throws),
        ) && matches(
            elsecf,
            control_flow_or(ControlFlow::Returns, ControlFlow::Throws),
        ) {
            ControlFlow::Returns
        } else {
            ControlFlow::None
        }
    }
}
