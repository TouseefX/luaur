use crate::records::module::Module;
use crate::records::source_module::SourceModule;
use crate::records::type_attacher::TypeAttacher;
use alloc::sync::Arc;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_stat_for::AstStatFor;
use luaur_ast::records::ast_stat_for_in::AstStatForIn;
use luaur_ast::records::ast_stat_local::AstStatLocal;
use luaur_ast::records::ast_visitor::AstVisitor;
use luaur_ast::visit::AstVisitable;

/// C++ `TypeAttacher : public AstVisitor`. The five overridden `visit`
/// overloads are landed as inherent methods on `TypeAttacher`; this impl
/// bridges them to the `AstVisitor` trait dispatch used by `root->visit(&ta)`.
impl AstVisitor for TypeAttacher {
    fn visit_stat_local(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_local(node as *mut AstStatLocal)
    }

    fn visit_expr_local(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_local(node as *mut AstExprLocal)
    }

    fn visit_stat_for(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_for(node as *mut AstStatFor)
    }

    fn visit_stat_for_in(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_for_in(node as *mut AstStatForIn)
    }

    fn visit_expr_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_function(node as *mut AstExprFunction)
    }
}

pub fn attach_type_data(source: &mut SourceModule, result: &mut Module) {
    // C++ `TypeAttacher ta(result, source.allocator.get()); source.root->visit(&ta);`
    let mut ta = TypeAttacher::type_attacher_type_attacher(
        result as *mut Module,
        Arc::as_ptr(&source.allocator) as *mut Allocator,
    );
    unsafe {
        (*source.root).visit(&mut ta);
    }
}
