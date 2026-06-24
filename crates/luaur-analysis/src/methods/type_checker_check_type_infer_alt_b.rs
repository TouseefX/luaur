use crate::enums::control_flow::ControlFlow;
use crate::records::generic_error::GenericError;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_assign::AstStatAssign;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::ast_stat_break::AstStatBreak;
use luaur_ast::records::ast_stat_class::AstStatClass;
use luaur_ast::records::ast_stat_compound_assign::AstStatCompoundAssign;
use luaur_ast::records::ast_stat_continue::AstStatContinue;
use luaur_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType;
use luaur_ast::records::ast_stat_declare_function::AstStatDeclareFunction;
use luaur_ast::records::ast_stat_declare_global::AstStatDeclareGlobal;
use luaur_ast::records::ast_stat_error::AstStatError;
use luaur_ast::records::ast_stat_expr::AstStatExpr;
use luaur_ast::records::ast_stat_for::AstStatFor;
use luaur_ast::records::ast_stat_for_in::AstStatForIn;
use luaur_ast::records::ast_stat_function::AstStatFunction;
use luaur_ast::records::ast_stat_if::AstStatIf;
use luaur_ast::records::ast_stat_local::AstStatLocal;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;
use luaur_ast::records::ast_stat_repeat::AstStatRepeat;
use luaur_ast::records::ast_stat_return::AstStatReturn;
use luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias;
use luaur_ast::records::ast_stat_type_function::AstStatTypeFunction;
use luaur_ast::records::ast_stat_while::AstStatWhile;
use luaur_ast::rtti::ast_node_as;

impl TypeChecker {
    pub fn check_scope_ptr_ast_stat(&mut self, scope: &ScopePtr, program: &AstStat) -> ControlFlow {
        let block =
            unsafe { ast_node_as::<AstStatBlock>(program as *const AstStat as *mut AstNode) };
        if !block.is_null() {
            return self.check_scope_ptr_ast_stat_block(scope, unsafe { &*block });
        }

        let if_ = unsafe { ast_node_as::<AstStatIf>(program as *const AstStat as *mut AstNode) };
        if !if_.is_null() {
            return self.check_scope_ptr_ast_stat_if(scope, unsafe { &*if_ });
        }

        let while_ =
            unsafe { ast_node_as::<AstStatWhile>(program as *const AstStat as *mut AstNode) };
        if !while_.is_null() {
            return self.check_scope_ptr_ast_stat_while(scope, unsafe { &*while_ });
        }

        let repeat =
            unsafe { ast_node_as::<AstStatRepeat>(program as *const AstStat as *mut AstNode) };
        if !repeat.is_null() {
            return self.check_scope_ptr_ast_stat_repeat(scope, unsafe { &*repeat });
        }

        let break_ =
            unsafe { ast_node_as::<AstStatBreak>(program as *const AstStat as *mut AstNode) };
        if !break_.is_null() {
            return ControlFlow::Breaks;
        }

        let continue_ =
            unsafe { ast_node_as::<AstStatContinue>(program as *const AstStat as *mut AstNode) };
        if !continue_.is_null() {
            return ControlFlow::Continues;
        }

        let return_ =
            unsafe { ast_node_as::<AstStatReturn>(program as *const AstStat as *mut AstNode) };
        if !return_.is_null() {
            return self.check_scope_ptr_ast_stat_return(scope, unsafe { &*return_ });
        }

        let expr = unsafe { ast_node_as::<AstStatExpr>(program as *const AstStat as *mut AstNode) };
        if !expr.is_null() {
            self.check_expr_pack(scope, unsafe { &*(*expr).expr });
            return ControlFlow::None;
        }

        let local =
            unsafe { ast_node_as::<AstStatLocal>(program as *const AstStat as *mut AstNode) };
        if !local.is_null() {
            return self.check_scope_ptr_ast_stat_local(scope, unsafe { &*local });
        }

        let for_ = unsafe { ast_node_as::<AstStatFor>(program as *const AstStat as *mut AstNode) };
        if !for_.is_null() {
            return self.check_scope_ptr_ast_stat_for(scope, unsafe { &*for_ });
        }

        let for_in =
            unsafe { ast_node_as::<AstStatForIn>(program as *const AstStat as *mut AstNode) };
        if !for_in.is_null() {
            return self.check_scope_ptr_ast_stat_for_in(scope, unsafe { &*for_in });
        }

        let assign =
            unsafe { ast_node_as::<AstStatAssign>(program as *const AstStat as *mut AstNode) };
        if !assign.is_null() {
            return self.check_scope_ptr_ast_stat_assign(scope, unsafe { &*assign });
        }

        let compound_assign = unsafe {
            ast_node_as::<AstStatCompoundAssign>(program as *const AstStat as *mut AstNode)
        };
        if !compound_assign.is_null() {
            return self
                .check_scope_ptr_ast_stat_compound_assign(scope, unsafe { &*compound_assign });
        }

        if !unsafe { ast_node_as::<AstStatFunction>(program as *const AstStat as *mut AstNode) }
            .is_null()
            || !unsafe {
                ast_node_as::<AstStatLocalFunction>(program as *const AstStat as *mut AstNode)
            }
            .is_null()
        {
            self.ice_string_location(
                "Should not be calling two-argument check() on a function statement",
                &program.base.location,
            );
            return ControlFlow::None;
        }

        let typealias =
            unsafe { ast_node_as::<AstStatTypeAlias>(program as *const AstStat as *mut AstNode) };

        if !typealias.is_null() {
            return self.check_scope_ptr_ast_stat_type_alias(scope, unsafe { &*typealias });
        }

        let typefunction = unsafe {
            ast_node_as::<AstStatTypeFunction>(program as *const AstStat as *mut AstNode)
        };

        if !typefunction.is_null() {
            return self.check_scope_ptr_ast_stat_type_function(scope, unsafe { &*typefunction });
        }

        let declare_global = unsafe {
            ast_node_as::<AstStatDeclareGlobal>(program as *const AstStat as *mut AstNode)
        };

        if !declare_global.is_null() {
            return self
                .check_scope_ptr_ast_stat_declare_global(scope, unsafe { &*declare_global });
        }

        let declare_function = unsafe {
            ast_node_as::<AstStatDeclareFunction>(program as *const AstStat as *mut AstNode)
        };

        if !declare_function.is_null() {
            return self
                .check_scope_ptr_ast_stat_declare_function(scope, unsafe { &*declare_function });
        }

        let declare_extern_type = unsafe {
            ast_node_as::<AstStatDeclareExternType>(program as *const AstStat as *mut AstNode)
        };

        if !declare_extern_type.is_null() {
            return self.check_scope_ptr_ast_stat_declare_extern_type(scope, unsafe {
                &*declare_extern_type
            });
        }

        let error_statement =
            unsafe { ast_node_as::<AstStatError>(program as *const AstStat as *mut AstNode) };

        if !error_statement.is_null() {
            return self.check_scope_ptr_ast_stat_error(scope, unsafe { &*error_statement });
        }

        let class_statement =
            unsafe { ast_node_as::<AstStatClass>(program as *const AstStat as *mut AstNode) };
        if luaur_common::FFlag::DebugLuauUserDefinedClasses.get() && !class_statement.is_null() {
            self.report_error_location_type_error_data(
                unsafe { &(*(*class_statement).name).location },
                TypeErrorData::GenericError(GenericError::new(alloc::string::String::from(
                    "class keyword is illegal here",
                ))),
            );
            return ControlFlow::None;
        }

        ControlFlow::None
    }
}
