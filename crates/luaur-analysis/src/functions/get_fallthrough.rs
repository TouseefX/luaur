use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_constant_bool::AstExprConstantBool;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::ast_stat_expr::AstStatExpr;
use luaur_ast::records::ast_stat_if::AstStatIf;
use luaur_ast::records::ast_stat_repeat::AstStatRepeat;
use luaur_ast::records::ast_stat_return::AstStatReturn;
use luaur_ast::records::ast_stat_while::AstStatWhile;
use luaur_ast::rtti::ast_node_as;
use luaur_ast::rtti::ast_node_is;

use crate::functions::does_call_error::does_call_error;
use crate::functions::has_break::has_break;

pub fn get_fallthrough(node: *const AstStat) -> *const AstStat {
    if node.is_null() {
        return core::ptr::null();
    }

    unsafe {
        if !ast_node_as::<AstStatBlock>(
            node as *const _ as *mut luaur_ast::records::ast_node::AstNode,
        )
        .is_null()
        {
            let stat = ast_node_as::<AstStatBlock>(
                node as *const _ as *mut luaur_ast::records::ast_node::AstNode,
            );
            if stat.is_null() {
                return core::ptr::null();
            }

            if (*stat).body.size == 0 {
                return stat as *const AstStat;
            }

            let size = (*stat).body.size;
            for i in 0..(size.saturating_sub(1)) {
                let s = *(*stat).body.data.add(i);
                if get_fallthrough(s).is_null() {
                    return core::ptr::null();
                }
            }

            let last = *(*stat).body.data.add(size - 1);
            return get_fallthrough(last);
        }

        if !ast_node_as::<AstStatIf>(node as *const _ as *mut luaur_ast::records::ast_node::AstNode)
            .is_null()
        {
            let stat = ast_node_as::<AstStatIf>(
                node as *const _ as *mut luaur_ast::records::ast_node::AstNode,
            );
            if stat.is_null() {
                return core::ptr::null();
            }

            let thenf = get_fallthrough((*stat).thenbody as *const AstStat);
            if !thenf.is_null() {
                return thenf;
            }

            if !(*stat).elsebody.is_null() {
                let elsef = get_fallthrough((*stat).elsebody);
                if !elsef.is_null() {
                    return elsef;
                }
                return core::ptr::null();
            } else {
                return node;
            }
        }

        if ast_node_is::<AstStatReturn>(unsafe {
            &*(node as *const luaur_ast::records::ast_node::AstNode)
        }) {
            return core::ptr::null();
        }

        if !ast_node_as::<AstStatExpr>(
            node as *const _ as *mut luaur_ast::records::ast_node::AstNode,
        )
        .is_null()
        {
            let stat = ast_node_as::<AstStatExpr>(
                node as *const _ as *mut luaur_ast::records::ast_node::AstNode,
            );
            if stat.is_null() {
                return core::ptr::null();
            }

            if !(*stat).expr.is_null() {
                let call = ast_node_as::<AstExprCall>(
                    (*stat).expr as *mut luaur_ast::records::ast_node::AstNode,
                );
                if !call.is_null() {
                    if does_call_error(&*call) {
                        return core::ptr::null();
                    }
                }
            }

            return node;
        }

        if !ast_node_as::<AstStatWhile>(
            node as *const _ as *mut luaur_ast::records::ast_node::AstNode,
        )
        .is_null()
        {
            let stat = ast_node_as::<AstStatWhile>(
                node as *const _ as *mut luaur_ast::records::ast_node::AstNode,
            );
            if stat.is_null() {
                return core::ptr::null();
            }

            if !(*stat).condition.is_null() {
                let expr = ast_node_as::<AstExprConstantBool>(
                    (*stat).condition as *mut luaur_ast::records::ast_node::AstNode,
                );
                if !expr.is_null() {
                    if (*expr).value && !has_break((*stat).body as *mut AstStat) {
                        return core::ptr::null();
                    }
                }
            }

            return node;
        }

        if !ast_node_as::<AstStatRepeat>(
            node as *const _ as *mut luaur_ast::records::ast_node::AstNode,
        )
        .is_null()
        {
            let stat = ast_node_as::<AstStatRepeat>(
                node as *const _ as *mut luaur_ast::records::ast_node::AstNode,
            );
            if stat.is_null() {
                return core::ptr::null();
            }

            if !(*stat).condition.is_null() {
                let expr = ast_node_as::<AstExprConstantBool>(
                    (*stat).condition as *mut luaur_ast::records::ast_node::AstNode,
                );
                if !expr.is_null() {
                    if !(*expr).value && !has_break((*stat).body as *mut AstStat) {
                        return core::ptr::null();
                    }
                }
            }

            if get_fallthrough((*stat).body as *const AstStat).is_null() {
                return core::ptr::null();
            }

            return node;
        }

        node
    }
}
