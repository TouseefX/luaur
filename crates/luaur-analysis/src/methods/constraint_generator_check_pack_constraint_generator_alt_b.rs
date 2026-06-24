use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::inference_pack::InferencePack;
use crate::records::module::Module;
use crate::records::recursion_counter::RecursionCounter;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_varargs::AstExprVarargs;
use luaur_ast::records::ast_node::AstNode;
use luaur_common::DFInt;
use luaur_common::LUAU_ASSERT;

impl ConstraintGenerator {
    pub fn check_pack_scope_ptr_ast_expr_vector_optional_type_id_bool(
        &mut self,
        scope: &ScopePtr,
        expr: *mut AstExpr,
        expected_types: &Vec<Option<TypeId>>,
        generalize: bool,
    ) -> InferencePack {
        let _counter = RecursionCounter::recursion_counter_i32(&mut self.recursion_count);

        if self.recursion_count >= DFInt::LuauConstraintGeneratorRecursionLimit.get() as i32 {
            self.report_code_too_complex(unsafe { (*expr).base.location });
            return InferencePack {
                tp: unsafe { (*self.builtin_types).errorTypePack },
                refinements: Vec::new(),
            };
        }

        let result: InferencePack;

        let node = expr as *mut AstNode;
        let call = unsafe { luaur_ast::rtti::ast_node_as::<AstExprCall>(node) };
        if !call.is_null() {
            result = self.check_pack_scope_ptr_ast_expr_call(scope, call);
        } else if unsafe { (*node).is::<AstExprVarargs>() } {
            if let Some(vararg_pack) = scope.as_ref().vararg_pack {
                result = InferencePack {
                    tp: vararg_pack,
                    refinements: Vec::new(),
                };
            } else {
                result = InferencePack {
                    tp: unsafe { (*self.builtin_types).errorTypePack },
                    refinements: Vec::new(),
                };
            }
        } else {
            let mut expected_type: Option<TypeId> = None;
            if !expected_types.is_empty() {
                expected_type = expected_types[0];
            }
            let t: TypeId = self
                .check_scope_ptr_ast_expr_optional_type_id_bool_bool(
                    scope,
                    expr,
                    expected_type,
                    false,
                    generalize,
                )
                .ty;
            result = InferencePack {
                tp: unsafe { (*self.arena).add_type_pack_initializer_list_type_id(&[t]) },
                refinements: Vec::new(),
            };
        }

        LUAU_ASSERT!(!result.tp.is_null());
        if let Some(module) = &self.module {
            let module_ptr = alloc::sync::Arc::as_ptr(module) as *mut Module;
            unsafe {
                *(*module_ptr)
                    .ast_type_packs
                    .get_or_insert(expr as *const AstExpr) = result.tp;
            }
        }
        result
    }
}
