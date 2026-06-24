use crate::records::function_info::FunctionInfo;
use crate::records::lint_global_local::LintGlobalLocal;
use core::ffi::c_void;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_stat_assign::AstStatAssign;
use luaur_ast::records::ast_stat_for::AstStatFor;
use luaur_ast::records::ast_stat_for_in::AstStatForIn;
use luaur_ast::records::ast_stat_function::AstStatFunction;
use luaur_ast::records::ast_stat_if::AstStatIf;
use luaur_ast::records::ast_stat_repeat::AstStatRepeat;
use luaur_ast::records::ast_stat_while::AstStatWhile;
use luaur_ast::records::ast_visitor::AstVisitor;

impl LintGlobalLocal {
    pub fn visit_ast_expr_function(&mut self, node: *mut AstExprFunction) -> bool {
        let node_body = unsafe { (*node).body };
        self.function_stack
            .push(FunctionInfo::function_info_ast(node));
        unsafe { luaur_ast::visit::ast_stat_block_visit(&*node_body, self) };
        self.function_stack.pop();
        false
    }
}

impl AstVisitor for LintGlobalLocal {
    fn visit_expr_function(&mut self, node: *mut c_void) -> bool {
        self.visit_ast_expr_function(node as *mut AstExprFunction)
    }

    fn visit_expr_global(&mut self, node: *mut c_void) -> bool {
        self.visit_ast_expr_global(node as *mut AstExprGlobal)
    }

    fn visit_expr_local(&mut self, node: *mut c_void) -> bool {
        self.visit_ast_expr_local(node as *mut AstExprLocal)
    }

    fn visit_stat_assign(&mut self, node: *mut c_void) -> bool {
        self.visit_ast_stat_assign(node as *mut AstStatAssign)
    }

    fn visit_stat_function(&mut self, node: *mut c_void) -> bool {
        self.visit_ast_stat_function(node as *mut AstStatFunction)
    }

    fn visit_stat_if(&mut self, node: *mut c_void) -> bool {
        self.visit_ast_stat_if(node as *mut AstStatIf)
    }

    fn visit_stat_while(&mut self, node: *mut c_void) -> bool {
        self.visit_ast_stat_while(node as *mut AstStatWhile)
    }

    fn visit_stat_repeat(&mut self, node: *mut c_void) -> bool {
        self.visit_ast_stat_repeat(node as *mut AstStatRepeat)
    }

    fn visit_stat_for(&mut self, node: *mut c_void) -> bool {
        self.visit_ast_stat_for(node as *mut AstStatFor)
    }

    fn visit_stat_for_in(&mut self, node: *mut c_void) -> bool {
        self.visit_ast_stat_for_in(node as *mut AstStatForIn)
    }

    fn visit_type(&mut self, _node: *mut c_void) -> bool {
        false
    }

    fn visit_type_pack(&mut self, _node: *mut c_void) -> bool {
        false
    }
}
