use crate::records::lint_implicit_return::LintImplicitReturn;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_return::AstStatReturn;
use luaur_ast::records::ast_visitor::AstVisitor;

use core::ffi::c_void;

pub fn lint_implicit_return_get_value_return(
    _this: &mut LintImplicitReturn,
    node: *mut c_void,
) -> *mut AstStatReturn {
    struct Visitor {
        result: *mut AstStatReturn,
    }

    impl AstVisitor for Visitor {
        fn visit_expr(&mut self, _node: *mut c_void) -> bool {
            false
        }

        fn visit_stat_return(&mut self, node: *mut c_void) -> bool {
            let node = node as *mut AstStatReturn;
            unsafe {
                if self.result.is_null() && (*node).list.size > 0 {
                    self.result = node;
                }
            }

            false
        }
    }

    let mut visitor = Visitor {
        result: core::ptr::null_mut(),
    };
    unsafe {
        luaur_ast::visit::ast_stat_visit(node as *mut AstStat, &mut visitor);
    }
    visitor.result
}
