use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_constant_bool::AstExprConstantBool;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::ast_stat_expr::AstStatExpr;
use luaur_ast::records::ast_stat_if::AstStatIf;
use luaur_ast::records::ast_stat_repeat::AstStatRepeat;
use luaur_ast::records::ast_stat_return::AstStatReturn;
use luaur_ast::records::ast_stat_while::AstStatWhile;
use luaur_ast::rtti;

impl TypeChecker2 {
    pub fn type_checker_2_get_fallthrough(&mut self, node: *const AstStat) -> *const AstStat {
        let node_ptr = node as *mut AstNode;

        let stat_block = unsafe { rtti::ast_node_as::<AstStatBlock>(node_ptr) };
        if !stat_block.is_null() {
            let stat = unsafe { &*stat_block };
            if stat.body.size == 0 {
                return node;
            }

            for i in 0..stat.body.size - 1 {
                let child = unsafe { *stat.body.data.add(i) };
                if self
                    .type_checker_2_get_fallthrough(child as *const AstStat)
                    .is_null()
                {
                    return core::ptr::null();
                }
            }

            let last = unsafe { *stat.body.data.add(stat.body.size - 1) };
            return self.type_checker_2_get_fallthrough(last as *const AstStat);
        }

        let stat_if = unsafe { rtti::ast_node_as::<AstStatIf>(node_ptr) };
        if !stat_if.is_null() {
            let stat = unsafe { &*stat_if };
            if let Some(thenf) = {
                let res = self.type_checker_2_get_fallthrough(stat.thenbody as *const AstStat);
                if res.is_null() {
                    None
                } else {
                    Some(res)
                }
            } {
                return thenf;
            }

            if !stat.elsebody.is_null() {
                if let Some(elsef) = {
                    let res = self.type_checker_2_get_fallthrough(stat.elsebody);
                    if res.is_null() {
                        None
                    } else {
                        Some(res)
                    }
                } {
                    return elsef;
                }
                return core::ptr::null();
            } else {
                return node;
            }
        }

        if unsafe { rtti::ast_node_is::<AstStatReturn>(&*node_ptr) } {
            return core::ptr::null();
        }

        let stat_expr = unsafe { rtti::ast_node_as::<AstStatExpr>(node_ptr) };
        if !stat_expr.is_null() {
            let stat = unsafe { &*stat_expr };
            let call = unsafe { rtti::ast_node_as::<AstExprCall>(stat.expr as *mut AstNode) };
            if !call.is_null() {
                if self.is_error_call(call) {
                    return core::ptr::null();
                }
            }
            return node;
        }

        let stat_while = unsafe { rtti::ast_node_as::<AstStatWhile>(node_ptr) };
        if !stat_while.is_null() {
            let stat = unsafe { &*stat_while };
            let expr =
                unsafe { rtti::ast_node_as::<AstExprConstantBool>(stat.condition as *mut AstNode) };
            if !expr.is_null() {
                let expr = unsafe { &*expr };
                if expr.value && !self.type_checker_2_has_break(stat.body as *mut AstStat) {
                    return core::ptr::null();
                }
            }
            return node;
        }

        let stat_repeat = unsafe { rtti::ast_node_as::<AstStatRepeat>(node_ptr) };
        if !stat_repeat.is_null() {
            let stat = unsafe { &*stat_repeat };
            let expr =
                unsafe { rtti::ast_node_as::<AstExprConstantBool>(stat.condition as *mut AstNode) };
            if !expr.is_null() {
                let expr = unsafe { &*expr };
                if !expr.value && !self.type_checker_2_has_break(stat.body as *mut AstStat) {
                    return core::ptr::null();
                }
            }
            if self
                .type_checker_2_get_fallthrough(stat.body as *const AstStat)
                .is_null()
            {
                return core::ptr::null();
            }
            return node;
        }

        node
    }
}
