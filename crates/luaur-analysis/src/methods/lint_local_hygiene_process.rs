use crate::records::global_linter_alt_c::Global;
use crate::records::lint_context::LintContext;
use crate::records::lint_local_hygiene::LintLocalHygiene;
use core::ffi::c_void;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_stat_assign::AstStatAssign;
use luaur_ast::records::ast_stat_local::AstStatLocal;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_pack::AstTypePack;
use luaur_ast::records::ast_type_reference::AstTypeReference;
use luaur_ast::records::ast_visitor::AstVisitor;

impl AstVisitor for LintLocalHygiene {
    fn visit_stat_assign(&mut self, node: *mut c_void) -> bool {
        self.visit_ast_stat_assign(node as *mut AstStatAssign)
    }

    fn visit_stat_local(&mut self, node: *mut c_void) -> bool {
        self.visit_ast_stat_local(node as *mut AstStatLocal)
    }

    fn visit_stat_local_function(&mut self, node: *mut c_void) -> bool {
        self.visit_ast_stat_local_function(node as *mut AstStatLocalFunction)
    }

    fn visit_expr_local(&mut self, node: *mut c_void) -> bool {
        self.visit_ast_expr_local(node as *mut AstExprLocal)
    }

    fn visit_expr_global(&mut self, node: *mut c_void) -> bool {
        self.visit_ast_expr_global(node as *mut AstExprGlobal)
    }

    fn visit_expr_function(&mut self, node: *mut c_void) -> bool {
        self.visit_ast_expr_function(node as *mut AstExprFunction)
    }

    fn visit_type(&mut self, node: *mut c_void) -> bool {
        self.visit_ast_type(node as *mut AstType)
    }

    fn visit_type_reference(&mut self, node: *mut c_void) -> bool {
        self.visit_ast_type_reference(node as *mut AstTypeReference)
    }

    fn visit_type_pack(&mut self, node: *mut c_void) -> bool {
        self.visit_ast_type_pack(node as *mut AstTypePack)
    }
}

pub fn lint_local_hygiene_process(context: &mut LintContext) {
    let mut pass = LintLocalHygiene::lint_local_hygiene();
    pass.context = context as *mut LintContext;

    for (global_name, global) in context.builtin_globals.iter() {
        let mut g = Global::default();
        g.builtin = true;
        let _ = global;
        pass.globals.try_insert(*global_name, g);
    }

    unsafe { luaur_ast::visit::ast_stat_visit(context.root, &mut pass) };
    pass.report();
}
