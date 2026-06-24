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
use luaur_ast::rtti::ast_node_as;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

#[allow(non_snake_case)]
pub fn similar(lhs: *mut AstExpr, rhs: *mut AstExpr) -> bool {
    if lhs.is_null() || rhs.is_null() {
        return false;
    }

    let lhs_ref = unsafe { &*lhs };
    let rhs_ref = unsafe { &*rhs };

    if lhs_ref.base.class_index != rhs_ref.base.class_index {
        return false;
    }

    let lhs_node = lhs as *mut AstNode;
    let rhs_node = rhs as *mut AstNode;

    let le_group = unsafe { ast_node_as::<AstExprGroup>(lhs_node) };
    if !le_group.is_null() {
        let re_group = unsafe { ast_node_as::<AstExprGroup>(rhs_node) };
        if !re_group.is_null() {
            return unsafe { similar((*le_group).expr, (*re_group).expr) };
        }
    }

    if !unsafe { ast_node_as::<AstExprConstantNil>(lhs_node) }.is_null() {
        return true;
    }

    let le_bool = unsafe { ast_node_as::<AstExprConstantBool>(lhs_node) };
    if !le_bool.is_null() {
        let re_bool = unsafe { ast_node_as::<AstExprConstantBool>(rhs_node) };
        if !re_bool.is_null() {
            return unsafe { (*le_bool).value == (*re_bool).value };
        }
    }

    let le_num = unsafe { ast_node_as::<AstExprConstantNumber>(lhs_node) };
    if !le_num.is_null() {
        let re_num = unsafe { ast_node_as::<AstExprConstantNumber>(rhs_node) };
        if !re_num.is_null() {
            return unsafe { (*le_num).value == (*re_num).value };
        }
    }

    let le_int = unsafe { ast_node_as::<AstExprConstantInteger>(lhs_node) };
    if !le_int.is_null() {
        let re_int = unsafe { ast_node_as::<AstExprConstantInteger>(rhs_node) };
        if !re_int.is_null() {
            return unsafe { (*le_int).value == (*re_int).value };
        }
    }

    let le_str = unsafe { ast_node_as::<AstExprConstantString>(lhs_node) };
    if !le_str.is_null() {
        let re_str = unsafe { ast_node_as::<AstExprConstantString>(rhs_node) };
        if !re_str.is_null() {
            let le_val = unsafe { (*le_str).value };
            let re_val = unsafe { (*re_str).value };
            if le_val.size != re_val.size {
                return false;
            }
            let le_slice = unsafe {
                core::slice::from_raw_parts(le_val.data as *const u8, le_val.size as usize)
            };
            let re_slice = unsafe {
                core::slice::from_raw_parts(re_val.data as *const u8, re_val.size as usize)
            };
            return le_slice == re_slice;
        }
    }

    let le_local = unsafe { ast_node_as::<AstExprLocal>(lhs_node) };
    if !le_local.is_null() {
        let re_local = unsafe { ast_node_as::<AstExprLocal>(rhs_node) };
        if !re_local.is_null() {
            return unsafe { (*le_local).local == (*re_local).local };
        }
    }

    let le_global = unsafe { ast_node_as::<AstExprGlobal>(lhs_node) };
    if !le_global.is_null() {
        let re_global = unsafe { ast_node_as::<AstExprGlobal>(rhs_node) };
        if !re_global.is_null() {
            return unsafe { (*le_global).name.value == (*re_global).name.value };
        }
    }

    if !unsafe { ast_node_as::<AstExprVarargs>(lhs_node) }.is_null() {
        return true;
    }

    let le_idx_name = unsafe { ast_node_as::<AstExprIndexName>(lhs_node) };
    if !le_idx_name.is_null() {
        let re_idx_name = unsafe { ast_node_as::<AstExprIndexName>(rhs_node) };
        if !re_idx_name.is_null() {
            return unsafe {
                (*le_idx_name).index.value == (*re_idx_name).index.value
                    && similar((*le_idx_name).expr, (*re_idx_name).expr)
            };
        }
    }

    let le_idx_expr = unsafe { ast_node_as::<AstExprIndexExpr>(lhs_node) };
    if !le_idx_expr.is_null() {
        let re_idx_expr = unsafe { ast_node_as::<AstExprIndexExpr>(rhs_node) };
        if !re_idx_expr.is_null() {
            return unsafe {
                similar((*le_idx_expr).expr, (*re_idx_expr).expr)
                    && similar((*le_idx_expr).index, (*re_idx_expr).index)
            };
        }
    }

    if !unsafe { ast_node_as::<AstExprFunction>(lhs_node) }.is_null() {
        return false;
    }

    let le_unary = unsafe { ast_node_as::<AstExprUnary>(lhs_node) };
    if !le_unary.is_null() {
        let re_unary = unsafe { ast_node_as::<AstExprUnary>(rhs_node) };
        if !re_unary.is_null() {
            return unsafe {
                (*le_unary).op == (*re_unary).op && similar((*le_unary).expr, (*re_unary).expr)
            };
        }
    }

    let le_binary = unsafe { ast_node_as::<AstExprBinary>(lhs_node) };
    if !le_binary.is_null() {
        let re_binary = unsafe { ast_node_as::<AstExprBinary>(rhs_node) };
        if !re_binary.is_null() {
            return unsafe {
                (*le_binary).op == (*re_binary).op
                    && similar((*le_binary).left, (*re_binary).left)
                    && similar((*le_binary).right, (*re_binary).right)
            };
        }
    }

    let le_assertion = unsafe { ast_node_as::<AstExprTypeAssertion>(lhs_node) };
    if !le_assertion.is_null() {
        let re_assertion = unsafe { ast_node_as::<AstExprTypeAssertion>(rhs_node) };
        if !re_assertion.is_null() {
            return unsafe { (*le_assertion).expr == (*re_assertion).expr };
        }
    }

    if !unsafe { ast_node_as::<AstExprError>(lhs_node) }.is_null() {
        return false;
    }

    let le_call = unsafe { ast_node_as::<AstExprCall>(lhs_node) };
    if !le_call.is_null() {
        let re_call = unsafe { ast_node_as::<AstExprCall>(rhs_node) };
        if !re_call.is_null() {
            let le = unsafe { &*le_call };
            let re = unsafe { &*re_call };
            if le.args.size != re.args.size || le.self_ != re.self_ {
                return false;
            }
            if !similar(le.func, re.func) {
                return false;
            }
            for i in 0..le.args.size {
                if !similar(unsafe { *le.args.data.add(i) }, unsafe {
                    *re.args.data.add(i)
                }) {
                    return false;
                }
            }
            return true;
        }
    }

    let le_table = unsafe { ast_node_as::<AstExprTable>(lhs_node) };
    if !le_table.is_null() {
        let re_table = unsafe { ast_node_as::<AstExprTable>(rhs_node) };
        if !re_table.is_null() {
            let le = unsafe { &*le_table };
            let re = unsafe { &*re_table };
            if le.items.size != re.items.size {
                return false;
            }
            for i in 0..le.items.size {
                let li = unsafe { &*le.items.data.add(i) };
                let ri = unsafe { &*re.items.data.add(i) };
                if li.kind != ri.kind {
                    return false;
                }
                if li.key.is_null() != ri.key.is_null() {
                    return false;
                }
                if !li.key.is_null() && !similar(li.key, ri.key) {
                    return false;
                }
                if !similar(li.value, ri.value) {
                    return false;
                }
            }
            return true;
        }
    }

    let le_ifelse = unsafe { ast_node_as::<AstExprIfElse>(lhs_node) };
    if !le_ifelse.is_null() {
        let re_ifelse = unsafe { ast_node_as::<AstExprIfElse>(rhs_node) };
        if !re_ifelse.is_null() {
            return unsafe {
                similar((*le_ifelse).condition, (*re_ifelse).condition)
                    && similar((*le_ifelse).true_expr, (*re_ifelse).true_expr)
                    && similar((*le_ifelse).false_expr, (*re_ifelse).false_expr)
            };
        }
    }

    let le_interp = unsafe { ast_node_as::<AstExprInterpString>(lhs_node) };
    if !le_interp.is_null() {
        let re_interp = unsafe { ast_node_as::<AstExprInterpString>(rhs_node) };
        if !re_interp.is_null() {
            let le = unsafe { &*le_interp };
            let re = unsafe { &*re_interp };
            if le.strings.size != re.strings.size || le.expressions.size != re.expressions.size {
                return false;
            }
            for i in 0..le.strings.size {
                let ls = unsafe { *le.strings.data.add(i) };
                let rs = unsafe { *re.strings.data.add(i) };
                if ls.size != rs.size {
                    return false;
                }
                let ls_slice =
                    unsafe { core::slice::from_raw_parts(ls.data as *const u8, ls.size as usize) };
                let rs_slice =
                    unsafe { core::slice::from_raw_parts(rs.data as *const u8, rs.size as usize) };
                if ls_slice != rs_slice {
                    return false;
                }
            }
            for i in 0..le.expressions.size {
                if !similar(unsafe { *le.expressions.data.add(i) }, unsafe {
                    *re.expressions.data.add(i)
                }) {
                    return false;
                }
            }
            return true;
        }
    }

    let le_inst = unsafe { ast_node_as::<AstExprInstantiate>(lhs_node) };
    if !le_inst.is_null() {
        let re_inst = unsafe { ast_node_as::<AstExprInstantiate>(rhs_node) };
        if !re_inst.is_null() {
            return unsafe { similar((*le_inst).expr, (*re_inst).expr) };
        }
    }

    LUAU_ASSERT!(false);
    false
}
