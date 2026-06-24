//! Hand-ported dispatcher: ConstraintGenerator::check(const ScopePtr&, AstExpr*, ...)
//! Source: `Analysis/src/ConstraintGenerator.cpp:3042`.
//!
//! The top-level expression `check` overload. C++ overloads `check` on the
//! static type of the expression; here we recover that with RTTI dispatch to
//! the per-node `check_scope_ptr_ast_expr_*` methods. Two convenience entry
//! points are provided to mirror the defaulted C++ arguments
//! (`expectedType = {}`, `forceSingleton = false`, `generalize = true`).
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::inference::Inference;
use crate::records::module::Module;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
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
use luaur_ast::records::ast_expr_table::AstExprTable;
use luaur_ast::records::ast_expr_type_assertion::AstExprTypeAssertion;
use luaur_ast::records::ast_expr_unary::AstExprUnary;
use luaur_ast::records::ast_expr_varargs::AstExprVarargs;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;
use luaur_common::DFInt;
use luaur_common::LUAU_ASSERT;

impl ConstraintGenerator {
    /// Convenience: `check(scope, expr)` with all C++ defaults.
    pub fn check_scope_ptr_ast_expr(&mut self, scope: &ScopePtr, expr: *mut AstExpr) -> Inference {
        self.check_scope_ptr_ast_expr_optional_type_id_bool_bool(scope, expr, None, false, true)
    }

    /// Convenience: `check(scope, expr, expectedType)`.
    pub fn check_scope_ptr_ast_expr_optional_type_id(
        &mut self,
        scope: &ScopePtr,
        expr: *mut AstExpr,
        expected_type: Option<TypeId>,
    ) -> Inference {
        self.check_scope_ptr_ast_expr_optional_type_id_bool_bool(
            scope,
            expr,
            expected_type,
            false,
            true,
        )
    }

    /// Convenience: `check(scope, expr, expectedType, forceSingleton)`.
    pub fn check_scope_ptr_ast_expr_optional_type_id_bool(
        &mut self,
        scope: &ScopePtr,
        expr: *mut AstExpr,
        expected_type: Option<TypeId>,
        force_singleton: bool,
    ) -> Inference {
        self.check_scope_ptr_ast_expr_optional_type_id_bool_bool(
            scope,
            expr,
            expected_type,
            force_singleton,
            true,
        )
    }

    pub fn check_scope_ptr_ast_expr_optional_type_id_bool_bool(
        &mut self,
        scope: &ScopePtr,
        expr: *mut AstExpr,
        expected_type: Option<TypeId>,
        force_singleton: bool,
        generalize: bool,
    ) -> Inference {
        // RecursionCounter counter{&recursionCount};
        self.recursion_count += 1;
        let result =
            self.check_dispatch_impl(scope, expr, expected_type, force_singleton, generalize);
        self.recursion_count -= 1;
        result
    }

    fn check_dispatch_impl(
        &mut self,
        scope: &ScopePtr,
        expr: *mut AstExpr,
        expected_type: Option<TypeId>,
        force_singleton: bool,
        generalize: bool,
    ) -> Inference {
        unsafe {
            if self.recursion_count >= DFInt::LuauConstraintGeneratorRecursionLimit.get() as i32 {
                self.report_code_too_complex((*expr).base.location);
                return Inference::inference_type_id_refinement_id(
                    (*self.builtin_types).errorType,
                    core::ptr::null_mut(),
                );
            }

            // We may recurse a given expression more than once when checking
            // compound assignment, so we store and cache expressions here.
            if self.inferred_expr_cache.contains(&expr) {
                return self.inferred_expr_cache.get_or_insert(expr).clone();
            }

            let node = expr as *mut AstNode;

            let result: Inference = {
                let group = ast_node_as::<AstExprGroup>(node);
                if !group.is_null() {
                    self.check_scope_ptr_ast_expr_optional_type_id_bool_bool(
                        scope,
                        (*group).expr,
                        expected_type,
                        force_singleton,
                        generalize,
                    )
                } else if !ast_node_as::<AstExprConstantString>(node).is_null() {
                    let string_expr = ast_node_as::<AstExprConstantString>(node);
                    self.check_scope_ptr_ast_expr_constant_string_optional_type_id_bool(
                        scope,
                        string_expr,
                        expected_type,
                        force_singleton,
                    )
                } else if !ast_node_as::<AstExprConstantNumber>(node).is_null() {
                    Inference::inference_type_id_refinement_id(
                        (*self.builtin_types).numberType,
                        core::ptr::null_mut(),
                    )
                } else if !ast_node_as::<AstExprConstantInteger>(node).is_null() {
                    Inference::inference_type_id_refinement_id(
                        (*self.builtin_types).integerType,
                        core::ptr::null_mut(),
                    )
                } else if !ast_node_as::<AstExprConstantBool>(node).is_null() {
                    let bool_expr = ast_node_as::<AstExprConstantBool>(node);
                    self.check_scope_ptr_ast_expr_constant_bool_optional_type_id_bool(
                        scope,
                        bool_expr,
                        expected_type,
                        force_singleton,
                    )
                } else if !ast_node_as::<AstExprConstantNil>(node).is_null() {
                    Inference::inference_type_id_refinement_id(
                        (*self.builtin_types).nilType,
                        core::ptr::null_mut(),
                    )
                } else if !ast_node_as::<AstExprLocal>(node).is_null() {
                    let local = ast_node_as::<AstExprLocal>(node);
                    self.check_scope_ptr_ast_expr_local(scope, local)
                } else if !ast_node_as::<AstExprGlobal>(node).is_null() {
                    let global = ast_node_as::<AstExprGlobal>(node);
                    self.check_scope_ptr_ast_expr_global(scope, global)
                } else if !ast_node_as::<AstExprVarargs>(node).is_null() {
                    let pack = self.check_pack_scope_ptr_ast_expr_vector_optional_type_id_bool(
                        scope,
                        expr,
                        &alloc::vec::Vec::new(),
                        true,
                    );
                    self.flatten_pack(scope, (*expr).base.location, pack)
                } else if !ast_node_as::<AstExprCall>(node).is_null() {
                    let call = ast_node_as::<AstExprCall>(node);
                    let pack = self.check_pack_scope_ptr_ast_expr_call(scope, call);
                    self.flatten_pack(scope, (*expr).base.location, pack)
                } else if !ast_node_as::<AstExprFunction>(node).is_null() {
                    let func = ast_node_as::<AstExprFunction>(node);
                    self.check_scope_ptr_ast_expr_function_optional_type_id_bool(
                        scope,
                        func,
                        expected_type,
                        generalize,
                    )
                } else if !ast_node_as::<AstExprIndexName>(node).is_null() {
                    let index_name = ast_node_as::<AstExprIndexName>(node);
                    self.check_scope_ptr_ast_expr_index_name(scope, index_name)
                } else if !ast_node_as::<AstExprIndexExpr>(node).is_null() {
                    let index_expr = ast_node_as::<AstExprIndexExpr>(node);
                    self.check_scope_ptr_ast_expr_index_expr(scope, index_expr)
                } else if !ast_node_as::<AstExprTable>(node).is_null() {
                    let table = ast_node_as::<AstExprTable>(node);
                    self.check_scope_ptr_ast_expr_table_optional_type_id(
                        scope,
                        table,
                        expected_type,
                    )
                } else if !ast_node_as::<AstExprUnary>(node).is_null() {
                    let unary = ast_node_as::<AstExprUnary>(node);
                    self.check_scope_ptr_ast_expr_unary(scope, unary)
                } else if !ast_node_as::<AstExprBinary>(node).is_null() {
                    // C++: check(scope, binary, expectedType) returns the full
                    // Inference (type + refinement). The
                    // `check_scope_ptr_ast_expr_binary_optional_type_id` wrapper
                    // discards the refinement (returns only `.ty`), so dispatch
                    // to `check_ast_expr_binary` directly to stay faithful.
                    let binary = ast_node_as::<AstExprBinary>(node);
                    self.check_ast_expr_binary(
                        scope,
                        (*binary).base.base.location,
                        (*binary).op,
                        (*binary).left,
                        (*binary).right,
                        expected_type,
                    )
                } else if !ast_node_as::<AstExprIfElse>(node).is_null() {
                    let if_else = ast_node_as::<AstExprIfElse>(node);
                    self.check_scope_ptr_ast_expr_if_else_optional_type_id(
                        scope,
                        if_else,
                        expected_type,
                    )
                } else if !ast_node_as::<AstExprTypeAssertion>(node).is_null() {
                    let type_assert = ast_node_as::<AstExprTypeAssertion>(node);
                    self.check_scope_ptr_ast_expr_type_assertion(scope, type_assert)
                } else if !ast_node_as::<AstExprInterpString>(node).is_null() {
                    let interp_string = ast_node_as::<AstExprInterpString>(node);
                    self.check_scope_ptr_ast_expr_interp_string(scope, interp_string)
                } else if !ast_node_as::<AstExprInstantiate>(node).is_null() {
                    let instantiate = ast_node_as::<AstExprInstantiate>(node);
                    self.check_scope_ptr_ast_expr_instantiate(scope, instantiate)
                } else {
                    let err = ast_node_as::<AstExprError>(node);
                    if !err.is_null() {
                        // Open question: Should we traverse into this?
                        let expressions = (*err).expressions;
                        for i in 0..expressions.size as usize {
                            let sub_expr = *expressions.data.add(i);
                            self.check_scope_ptr_ast_expr(scope, sub_expr);
                        }
                        Inference::inference_type_id_refinement_id(
                            (*self.builtin_types).errorType,
                            core::ptr::null_mut(),
                        )
                    } else {
                        LUAU_ASSERT!(false);
                        Inference::inference_type_id_refinement_id(
                            self.fresh_type(scope, self.polarity),
                            core::ptr::null_mut(),
                        )
                    }
                }
            };

            *self.inferred_expr_cache.get_or_insert(expr) = result.clone();

            LUAU_ASSERT!(!result.ty.is_null());

            if let Some(module) = &self.module {
                let module_ptr = alloc::sync::Arc::as_ptr(module) as *mut Module;
                *(*module_ptr)
                    .ast_types
                    .get_or_insert(expr as *const AstExpr) = result.ty;
                if let Some(et) = expected_type {
                    *(*module_ptr)
                        .ast_expected_types
                        .get_or_insert(expr as *const AstExpr) = et;
                }
            }

            result
        }
    }
}

use luaur_ast::records::ast_expr_local::AstExprLocal;
