use crate::functions::try_get_l_value::try_get_l_value;
use crate::records::not_predicate::NotPredicate;
use crate::records::type_guard_predicate::TypeGuardPredicate;
use crate::type_aliases::predicate::Predicate;
use crate::type_aliases::predicate_vec::PredicateVec;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_binary::{AstExprBinary, AstExprBinary_Op};
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;

pub fn try_get_type_guard_predicate(expr: &AstExprBinary) -> Option<Predicate> {
    if expr.op != AstExprBinary_Op::CompareEq && expr.op != AstExprBinary_Op::CompareNe {
        return None;
    }

    let mut left: *mut AstExpr = expr.left;
    let mut right: *mut AstExpr = expr.right;

    unsafe {
        if !ast_node_as::<AstExprConstantString>(left as *mut AstNode).is_null() {
            core::mem::swap(&mut left, &mut right);
        }

        let str_node = ast_node_as::<AstExprConstantString>(right as *mut AstNode);
        if str_node.is_null() {
            return None;
        }

        let call = ast_node_as::<AstExprCall>(left as *mut AstNode);
        if call.is_null() {
            return None;
        }

        let callee = ast_node_as::<AstExprGlobal>((*call).func as *mut AstNode);
        if callee.is_null() {
            return None;
        }

        // C++ `AstName::operator==(const char*)` strcmps the interned value (false when null).
        let name_ptr = (*callee).name.value;
        if name_ptr.is_null() {
            return None;
        }
        let callee_name = core::ffi::CStr::from_ptr(name_ptr).to_bytes();
        if callee_name != b"type" && callee_name != b"typeof" {
            return None;
        }

        if (*call).args.size != 1 {
            return None;
        }

        // If ssval is not a valid constant string, we'll find out later when resolving predicate.
        let ssval: alloc::string::String = {
            let data = (*str_node).value.data;
            let size = (*str_node).value.size;
            if data.is_null() {
                alloc::string::String::new()
            } else {
                let slice = core::slice::from_raw_parts(data as *const u8, size as usize);
                alloc::string::String::from_utf8_lossy(slice).into_owned()
            }
        };
        let is_typeof = callee_name == b"typeof";

        let arg0 = *(*call).args.data.add(0);
        let lvalue = match try_get_l_value(&*arg0) {
            Some(lv) => lv,
            None => return None,
        };

        let predicate = Predicate::TypeGuard(TypeGuardPredicate {
            lvalue,
            location: expr.base.base.location,
            kind: ssval,
            is_typeof,
        });

        if expr.op == AstExprBinary_Op::CompareNe {
            return Some(Predicate::Not(NotPredicate {
                predicates: PredicateVec::from(alloc::vec![predicate]),
            }));
        }

        Some(predicate)
    }
}
