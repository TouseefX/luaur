use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::data_flow_result::DataFlowResult;
use crate::records::def::Def;
use crate::records::symbol::Symbol;
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

impl DataFlowGraphBuilder {
    pub fn visit_expr_ast_expr(&mut self, e: *mut AstExpr) -> DataFlowResult {
        unsafe {
            if let Some(def) = self.graph.ast_defs.find(&(e as *const AstExpr)) {
                let key = self.graph.ast_refinement_keys.find(&(e as *const AstExpr));
                return DataFlowResult {
                    def: *def as *const core::ffi::c_void,
                    parent: if let Some(k) = key {
                        *k
                    } else {
                        core::ptr::null()
                    },
                };
            }

            let result = {
                let node = e as *mut AstNode;
                if (*node).is::<AstExprGroup>() {
                    self.visit_expr_ast_expr_group(e as *mut AstExprGroup)
                } else if (*node).is::<AstExprConstantNil>() {
                    DataFlowResult {
                        def: (*self.def_arena).fresh_cell(
                            Symbol::default(),
                            (*node).location,
                            false,
                        ) as *const core::ffi::c_void,
                        parent: core::ptr::null(),
                    }
                } else if (*node).is::<AstExprConstantBool>() {
                    DataFlowResult {
                        def: (*self.def_arena).fresh_cell(
                            Symbol::default(),
                            (*node).location,
                            false,
                        ) as *const core::ffi::c_void,
                        parent: core::ptr::null(),
                    }
                } else if (*node).is::<AstExprConstantNumber>() {
                    DataFlowResult {
                        def: (*self.def_arena).fresh_cell(
                            Symbol::default(),
                            (*node).location,
                            false,
                        ) as *const core::ffi::c_void,
                        parent: core::ptr::null(),
                    }
                } else if (*node).is::<AstExprConstantInteger>() {
                    DataFlowResult {
                        def: (*self.def_arena).fresh_cell(
                            Symbol::default(),
                            (*node).location,
                            false,
                        ) as *const core::ffi::c_void,
                        parent: core::ptr::null(),
                    }
                } else if (*node).is::<AstExprConstantString>() {
                    DataFlowResult {
                        def: (*self.def_arena).fresh_cell(
                            Symbol::default(),
                            (*node).location,
                            false,
                        ) as *const core::ffi::c_void,
                        parent: core::ptr::null(),
                    }
                } else if (*node).is::<AstExprLocal>() {
                    self.visit_expr_ast_expr_local(e as *mut AstExprLocal)
                } else if (*node).is::<AstExprGlobal>() {
                    self.visit_expr_ast_expr_global(e as *mut AstExprGlobal)
                } else if (*node).is::<AstExprVarargs>() {
                    DataFlowResult {
                        def: (*self.def_arena).fresh_cell(
                            Symbol::default(),
                            (*node).location,
                            false,
                        ) as *const core::ffi::c_void,
                        parent: core::ptr::null(),
                    }
                } else if (*node).is::<AstExprCall>() {
                    self.visit_expr_ast_expr_call(e as *mut AstExprCall)
                } else if (*node).is::<AstExprIndexName>() {
                    self.visit_expr_ast_expr_index_name(e as *mut AstExprIndexName)
                } else if (*node).is::<AstExprIndexExpr>() {
                    self.visit_expr_ast_expr_index_expr(e as *mut AstExprIndexExpr)
                } else if (*node).is::<AstExprFunction>() {
                    self.visit_expr_ast_expr_function(e as *mut AstExprFunction)
                } else if (*node).is::<AstExprTable>() {
                    self.visit_expr_ast_expr_table(e as *mut AstExprTable)
                } else if (*node).is::<AstExprUnary>() {
                    self.visit_expr_ast_expr_unary(e as *mut AstExprUnary)
                } else if (*node).is::<AstExprBinary>() {
                    self.visit_expr_ast_expr_binary(e as *mut AstExprBinary)
                } else if (*node).is::<AstExprTypeAssertion>() {
                    self.visit_expr_ast_expr_type_assertion(e as *mut AstExprTypeAssertion)
                } else if (*node).is::<AstExprIfElse>() {
                    self.visit_expr_ast_expr_if_else(e as *mut AstExprIfElse)
                } else if (*node).is::<AstExprInterpString>() {
                    self.visit_expr_ast_expr_interp_string(e as *mut AstExprInterpString)
                } else if (*node).is::<AstExprInstantiate>() {
                    self.visit_expr_ast_expr_instantiate(e as *mut AstExprInstantiate)
                } else if (*node).is::<AstExprError>() {
                    self.visit_expr_ast_expr_error(e as *mut AstExprError)
                } else {
                    (*self.handle).ice_string("Unknown AstExpr in DataFlowGraphBuilder::visitExpr");
                    DataFlowResult::default()
                }
            };

            *self.graph.ast_defs.get_or_insert(e as *const AstExpr) = result.def as *const Def;
            if !result.parent.is_null() {
                *self
                    .graph
                    .ast_refinement_keys
                    .get_or_insert(e as *const AstExpr) = result.parent;
            }

            result
        }
    }
}
