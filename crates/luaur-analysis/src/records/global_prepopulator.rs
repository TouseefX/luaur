use crate::records::data_flow_graph::DataFlowGraph;
use crate::records::scope::Scope;
use crate::records::type_arena::TypeArena;
use core::ptr::NonNull;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_visitor::AstVisitor;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct GlobalPrepopulator {
    pub global_scope: NonNull<Scope>,
    pub arena: NonNull<TypeArena>,
    pub dfg: NonNull<DataFlowGraph>,
    pub uninitialized_globals: DenseHashSet<AstName>,
}

impl AstVisitor for GlobalPrepopulator {
    fn visit_expr_global(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_global(node as *mut _)
    }

    fn visit_type(&mut self, _node: *mut core::ffi::c_void) -> bool {
        true
    }

    fn visit_type_pack(&mut self, _node: *mut core::ffi::c_void) -> bool {
        true
    }

    fn visit_stat_assign(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_assign(node as *mut _)
    }
    fn visit_stat_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_function(node as *mut _)
    }
}
