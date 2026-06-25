//! Source: `Analysis/src/AstJsonEncoder.cpp:1174-1518` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_visitor::AstVisitor;

// The C++ encoder's 54 `bool visit(class AstX*) override`s. Each trait hook
// receives the type-erased node pointer the dispatcher hands out and forwards
// to the typed inherent method (the per-override graph item). NB: C++ does NOT
// override visit(AstExprConstantInteger*) -- integer constants fall through to
// the AstNode default (return true), so neither do we.
impl AstVisitor for AstJsonEncoder {
    fn visit_type_group(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type_group(node as *mut luaur_ast::records::ast_type_group::AstTypeGroup)
    }

    fn visit_type_singleton_bool(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type_singleton_bool(
            node as *mut luaur_ast::records::ast_type_singleton_bool::AstTypeSingletonBool,
        )
    }

    fn visit_type_singleton_string(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type_singleton_string(
            node as *mut luaur_ast::records::ast_type_singleton_string::AstTypeSingletonString,
        )
    }

    fn visit_expr_group(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_group(node as *mut luaur_ast::records::ast_expr_group::AstExprGroup)
    }

    fn visit_expr_constant_nil(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_constant_nil(
            node as *mut luaur_ast::records::ast_expr_constant_nil::AstExprConstantNil,
        )
    }

    fn visit_expr_constant_bool(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_constant_bool(
            node as *mut luaur_ast::records::ast_expr_constant_bool::AstExprConstantBool,
        )
    }

    fn visit_expr_constant_number(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_constant_number(
            node as *mut luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber,
        )
    }

    fn visit_expr_constant_string(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_constant_string(
            node as *mut luaur_ast::records::ast_expr_constant_string::AstExprConstantString,
        )
    }

    fn visit_expr_if_else(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_if_else(
            node as *mut luaur_ast::records::ast_expr_if_else::AstExprIfElse,
        )
    }

    fn visit_expr_interp_string(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_interp_string(
            node as *mut luaur_ast::records::ast_expr_interp_string::AstExprInterpString,
        )
    }

    fn visit_expr_local(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_local(node as *mut luaur_ast::records::ast_expr_local::AstExprLocal)
    }

    fn visit_expr_global(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_global(node as *mut luaur_ast::records::ast_expr_global::AstExprGlobal)
    }

    fn visit_expr_varargs(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_varargs(
            node as *mut luaur_ast::records::ast_expr_varargs::AstExprVarargs,
        )
    }

    fn visit_expr_call(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_call(node as *mut luaur_ast::records::ast_expr_call::AstExprCall)
    }

    fn visit_expr_index_name(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_index_name(
            node as *mut luaur_ast::records::ast_expr_index_name::AstExprIndexName,
        )
    }

    fn visit_expr_index_expr(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_index_expr(
            node as *mut luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr,
        )
    }

    fn visit_expr_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_function(
            node as *mut luaur_ast::records::ast_expr_function::AstExprFunction,
        )
    }

    fn visit_expr_table(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_table(node as *mut luaur_ast::records::ast_expr_table::AstExprTable)
    }

    fn visit_expr_unary(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_unary(node as *mut luaur_ast::records::ast_expr_unary::AstExprUnary)
    }

    fn visit_expr_binary(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_binary(node as *mut luaur_ast::records::ast_expr_binary::AstExprBinary)
    }

    fn visit_expr_type_assertion(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_type_assertion(
            node as *mut luaur_ast::records::ast_expr_type_assertion::AstExprTypeAssertion,
        )
    }

    fn visit_expr_error(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_error(node as *mut luaur_ast::records::ast_expr_error::AstExprError)
    }

    fn visit_stat_block(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_block(node as *mut luaur_ast::records::ast_stat_block::AstStatBlock)
    }

    fn visit_stat_if(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_if(node as *mut luaur_ast::records::ast_stat_if::AstStatIf)
    }

    fn visit_stat_while(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_while(node as *mut luaur_ast::records::ast_stat_while::AstStatWhile)
    }

    fn visit_stat_repeat(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_repeat(node as *mut luaur_ast::records::ast_stat_repeat::AstStatRepeat)
    }

    fn visit_stat_break(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_break(node as *mut luaur_ast::records::ast_stat_break::AstStatBreak)
    }

    fn visit_stat_continue(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_continue(
            node as *mut luaur_ast::records::ast_stat_continue::AstStatContinue,
        )
    }

    fn visit_stat_return(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_return(node as *mut luaur_ast::records::ast_stat_return::AstStatReturn)
    }

    fn visit_stat_expr(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_expr(node as *mut luaur_ast::records::ast_stat_expr::AstStatExpr)
    }

    fn visit_stat_local(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_local(node as *mut luaur_ast::records::ast_stat_local::AstStatLocal)
    }

    fn visit_stat_for(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_for(node as *mut luaur_ast::records::ast_stat_for::AstStatFor)
    }

    fn visit_stat_for_in(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_for_in(node as *mut luaur_ast::records::ast_stat_for_in::AstStatForIn)
    }

    fn visit_stat_assign(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_assign(node as *mut luaur_ast::records::ast_stat_assign::AstStatAssign)
    }

    fn visit_stat_compound_assign(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_compound_assign(
            node as *mut luaur_ast::records::ast_stat_compound_assign::AstStatCompoundAssign,
        )
    }

    fn visit_stat_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_function(
            node as *mut luaur_ast::records::ast_stat_function::AstStatFunction,
        )
    }

    fn visit_stat_local_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_local_function(
            node as *mut luaur_ast::records::ast_stat_local_function::AstStatLocalFunction,
        )
    }

    fn visit_stat_type_alias(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_type_alias(
            node as *mut luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias,
        )
    }

    fn visit_stat_declare_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_declare_function(
            node as *mut luaur_ast::records::ast_stat_declare_function::AstStatDeclareFunction,
        )
    }

    fn visit_stat_declare_global(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_declare_global(
            node as *mut luaur_ast::records::ast_stat_declare_global::AstStatDeclareGlobal,
        )
    }

    fn visit_stat_declare_extern_type(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_declare_extern_type(
            node as *mut luaur_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType,
        )
    }

    fn visit_stat_error(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_error(node as *mut luaur_ast::records::ast_stat_error::AstStatError)
    }

    fn visit_type_reference(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type_reference(
            node as *mut luaur_ast::records::ast_type_reference::AstTypeReference,
        )
    }

    fn visit_type_table(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type_table(node as *mut luaur_ast::records::ast_type_table::AstTypeTable)
    }

    fn visit_type_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type_function(
            node as *mut luaur_ast::records::ast_type_function::AstTypeFunction,
        )
    }

    fn visit_type_typeof(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type_typeof(node as *mut luaur_ast::records::ast_type_typeof::AstTypeTypeof)
    }

    fn visit_type_optional(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type_optional(
            node as *mut luaur_ast::records::ast_type_optional::AstTypeOptional,
        )
    }

    fn visit_type_union(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type_union(node as *mut luaur_ast::records::ast_type_union::AstTypeUnion)
    }

    fn visit_type_intersection(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type_intersection(
            node as *mut luaur_ast::records::ast_type_intersection::AstTypeIntersection,
        )
    }

    fn visit_type_error(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type_error(node as *mut luaur_ast::records::ast_type_error::AstTypeError)
    }

    fn visit_type_pack(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type_pack(node as *mut luaur_ast::records::ast_type_pack::AstTypePack)
    }

    fn visit_type_pack_explicit(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type_pack_explicit(
            node as *mut luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit,
        )
    }

    fn visit_type_pack_variadic(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type_pack_variadic(
            node as *mut luaur_ast::records::ast_type_pack_variadic::AstTypePackVariadic,
        )
    }

    fn visit_type_pack_generic(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type_pack_generic(
            node as *mut luaur_ast::records::ast_type_pack_generic::AstTypePackGeneric,
        )
    }
}
