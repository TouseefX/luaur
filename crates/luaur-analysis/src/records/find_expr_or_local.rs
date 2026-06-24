use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_local::AstLocal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::ast_stat_for::AstStatFor;
use luaur_ast::records::ast_stat_for_in::AstStatForIn;
use luaur_ast::records::ast_stat_local::AstStatLocal;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;
use luaur_ast::records::ast_visitor::AstVisitor;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;
use luaur_ast::visit;

use crate::records::expr_or_local::ExprOrLocal;

#[derive(Debug, Clone)]
pub struct FindExprOrLocal {
    pub(crate) pos: Position,
    pub(crate) result: ExprOrLocal,
}

impl FindExprOrLocal {
    pub fn new(pos: Position) -> Self {
        Self {
            pos,
            result: ExprOrLocal {
                expr: core::ptr::null_mut(),
                local: core::ptr::null_mut(),
            },
        }
    }

    pub(crate) fn is_closer_match(&self, new_location: Location) -> bool {
        let current = self.result.get_location();
        new_location.contains(self.pos)
            && (current.is_none() || current.map_or(false, |c| c.encloses(&new_location)))
    }

    fn visit_local(&mut self, local: *mut AstLocal) -> bool {
        let location = unsafe { (*local).location };
        if self.is_closer_match(location) {
            self.result.set_local(local);
            true
        } else {
            false
        }
    }
}

impl AstVisitor for FindExprOrLocal {
    fn visit_stat_block(&mut self, node: *mut core::ffi::c_void) -> bool {
        let block = node as *mut AstStatBlock;
        unsafe {
            for stat in (*block).body.iter() {
                let stat = *stat;
                let stat_node = stat as *mut AstNode;
                if (*stat_node).location.end <= self.pos {
                    continue;
                }
                if (*stat_node).location.begin > self.pos {
                    break;
                }
                visit::ast_stat_visit(stat, self);
            }
        }
        false
    }

    fn visit_expr(&mut self, node: *mut core::ffi::c_void) -> bool {
        let expr = node as *mut AstExpr;
        let location = unsafe { (*expr).base.location };
        if self.is_closer_match(location) {
            self.result.set_expr(expr);
            true
        } else {
            false
        }
    }

    fn visit_stat_local_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        let func = node as *mut AstStatLocalFunction;
        unsafe {
            self.visit_local((*func).name);
        }
        true
    }

    fn visit_stat_local(&mut self, node: *mut core::ffi::c_void) -> bool {
        let stat = node as *mut AstStatLocal;
        unsafe {
            for i in 0..(*stat).vars.size {
                let local = unsafe { *((*stat).vars.data.add(i as usize)) };
                self.visit_local(local);
            }
        }
        true
    }

    fn visit_expr_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        let func = node as *mut AstExprFunction;
        unsafe {
            for i in 0..(*func).args.size {
                let local = unsafe { *((*func).args.data.add(i as usize)) };
                self.visit_local(local);
            }
        }
        self.visit_expr(node)
    }

    fn visit_stat_for(&mut self, node: *mut core::ffi::c_void) -> bool {
        let stat = node as *mut AstStatFor;
        unsafe {
            self.visit_local((*stat).var);
        }
        true
    }

    fn visit_stat_for_in(&mut self, node: *mut core::ffi::c_void) -> bool {
        let stat = node as *mut AstStatForIn;
        unsafe {
            for i in 0..(*stat).vars.size {
                let local = unsafe { *((*stat).vars.data.add(i as usize)) };
                self.visit_local(local);
            }
        }
        true
    }
}
