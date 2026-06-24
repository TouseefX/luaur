use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

use crate::functions::similar::similar;

pub fn case_ast_expr_call(left: *mut AstExprCall, right: *mut AstExprCall) -> bool {
    let le = unsafe { &*left };
    let re = unsafe { &*right };

    if le.args.size != re.args.size || le.self_ != re.self_ {
        return false;
    }

    if !similar(le.func, re.func) {
        return false;
    }

    for i in 0..le.args.size {
        let left_arg = unsafe { *le.args.data.add(i as usize) };
        let right_arg = unsafe { *re.args.data.add(i as usize) };
        if !similar(left_arg, right_arg) {
            return false;
        }
    }

    true
}
