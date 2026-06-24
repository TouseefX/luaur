use crate::enums::value_context::ValueContext;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use crate::records::recursion_counter::RecursionCounter;
use crate::records::stack_pusher_non_strict_type_checker::StackPusher;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_binary::AstExprBinary;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_constant_bool::AstExprConstantBool;
use luaur_ast::records::ast_expr_constant_integer::AstExprConstantInteger;
use luaur_ast::records::ast_expr_constant_nil::AstExprConstantNil;
use luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_error::AstExprError;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_group::AstExprGroup;
use luaur_ast::records::ast_expr_if_else::AstExprIfElse;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_expr_instantiate::AstExprInstantiate;
use luaur_ast::records::ast_expr_interp_string::AstExprInterpString;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_expr_table::AstExprTable;
use luaur_ast::records::ast_expr_type_assertion::AstExprTypeAssertion;
use luaur_ast::records::ast_expr_unary::AstExprUnary;
use luaur_ast::records::ast_expr_varargs::AstExprVarargs;
use luaur_ast::records::ast_node::AstNode;
use luaur_common::FFlag;
use luaur_common::FInt;
use luaur_common::LUAU_ASSERT;

impl NonStrictTypeChecker {
    pub fn visit_ast_expr_value_context(
        &mut self,
        expr: *mut AstExpr,
        context: ValueContext,
    ) -> NonStrictContext {
        let mut _rc: Option<RecursionCounter> = None;
        if FFlag::LuauAddRecursionCounterToNonStrictTypeChecker.get() {
            _rc = Some(RecursionCounter::recursion_counter_i32(
                &mut self.non_strict_recursion_count,
            ));
            if FInt::LuauNonStrictTypeCheckerRecursionLimit.get() > 0
                && self.non_strict_recursion_count
                    >= FInt::LuauNonStrictTypeCheckerRecursionLimit.get()
            {
                return NonStrictContext::non_strict_context();
            }
        }

        let _pusher = self.push_stack(expr as *mut AstNode);

        let expr_ptr = expr as *mut AstNode;
        let class_index = unsafe { (*expr_ptr).class_index };

        if class_index == crate::rtti::ast_rtti_index("AstExprGroup") {
            unsafe { crate::rtti::ast_node_as::<AstExprGroup>(expr_ptr) };
            let e = unsafe { &mut *(expr as *mut AstExprGroup) };
            return self.visit_ast_expr_group_value_context(e, context);
        } else if class_index == crate::rtti::ast_rtti_index("AstExprConstantNil") {
            let e = unsafe { &mut *(expr as *mut AstExprConstantNil) };
            return self.visit_ast_expr_constant_nil(e);
        } else if class_index == crate::rtti::ast_rtti_index("AstExprConstantBool") {
            let e = unsafe { &mut *(expr as *mut AstExprConstantBool) };
            return self.visit_ast_expr_constant_bool(e);
        } else if class_index == crate::rtti::ast_rtti_index("AstExprConstantNumber") {
            let e = unsafe { &mut *(expr as *mut AstExprConstantNumber) };
            return self.visit_ast_expr_constant_number(e);
        } else if class_index == crate::rtti::ast_rtti_index("AstExprConstantInteger") {
            let e = unsafe { &mut *(expr as *mut AstExprConstantInteger) };
            return self.visit_ast_expr_constant_integer(e);
        } else if class_index == crate::rtti::ast_rtti_index("AstExprConstantString") {
            let e = unsafe { &mut *(expr as *mut AstExprConstantString) };
            return self.visit_ast_expr_constant_string(e);
        } else if class_index == crate::rtti::ast_rtti_index("AstExprLocal") {
            let e = unsafe { &mut *(expr as *mut AstExprLocal) };
            return self.visit_ast_expr_local_value_context(e, context);
        } else if class_index == crate::rtti::ast_rtti_index("AstExprGlobal") {
            let e = unsafe { &mut *(expr as *mut AstExprGlobal) };
            return self.visit_ast_expr_global_value_context(e, context);
        } else if class_index == crate::rtti::ast_rtti_index("AstExprVarargs") {
            let e = unsafe { &mut *(expr as *mut AstExprVarargs) };
            return self.visit_ast_expr_varargs(e);
        } else if class_index == crate::rtti::ast_rtti_index("AstExprCall") {
            let e = unsafe { &mut *(expr as *mut AstExprCall) };
            return self.visit_ast_expr_call(e);
        } else if class_index == crate::rtti::ast_rtti_index("AstExprIndexName") {
            let e = unsafe { &mut *(expr as *mut AstExprIndexName) };
            return self.visit_ast_expr_index_name_value_context(e, context);
        } else if class_index == crate::rtti::ast_rtti_index("AstExprIndexExpr") {
            let e = unsafe { &mut *(expr as *mut AstExprIndexExpr) };
            return self.visit_ast_expr_index_expr_value_context(e, context);
        } else if class_index == crate::rtti::ast_rtti_index("AstExprFunction") {
            let e = unsafe { &mut *(expr as *mut AstExprFunction) };
            return self.visit_ast_expr_function(e);
        } else if class_index == crate::rtti::ast_rtti_index("AstExprTable") {
            let e = unsafe { &mut *(expr as *mut AstExprTable) };
            return self.visit_ast_expr_table(e);
        } else if class_index == crate::rtti::ast_rtti_index("AstExprUnary") {
            let e = unsafe { &mut *(expr as *mut AstExprUnary) };
            return self.visit_ast_expr_unary(e);
        } else if class_index == crate::rtti::ast_rtti_index("AstExprBinary") {
            let e = unsafe { &mut *(expr as *mut AstExprBinary) };
            return self.visit_ast_expr_binary(e);
        } else if class_index == crate::rtti::ast_rtti_index("AstExprTypeAssertion") {
            let e = unsafe { &mut *(expr as *mut AstExprTypeAssertion) };
            return self.visit_ast_expr_type_assertion(e);
        } else if class_index == crate::rtti::ast_rtti_index("AstExprIfElse") {
            let e = unsafe { &mut *(expr as *mut AstExprIfElse) };
            return self.visit_ast_expr_if_else(e);
        } else if class_index == crate::rtti::ast_rtti_index("AstExprInterpString") {
            let e = unsafe { &mut *(expr as *mut AstExprInterpString) };
            return self.visit_ast_expr_interp_string(e);
        } else if class_index == crate::rtti::ast_rtti_index("AstExprError") {
            let e = unsafe { &mut *(expr as *mut AstExprError) };
            return self.visit_ast_expr_error(e);
        } else if class_index == crate::rtti::ast_rtti_index("AstExprInstantiate") {
            let e = unsafe { &mut *(expr as *mut AstExprInstantiate) };
            return self.visit_ast_expr_instantiate(e);
        } else {
            LUAU_ASSERT!(false);
            unsafe {
                (*self.ice)
                    .ice_string("NonStrictTypeChecker encountered an unknown expression type")
            };
            return NonStrictContext::non_strict_context();
        }
    }
}
