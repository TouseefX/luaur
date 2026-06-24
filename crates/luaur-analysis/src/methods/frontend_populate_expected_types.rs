use crate::records::expected_type_visitor::ExpectedTypeVisitor;
use crate::records::frontend::Frontend;
use crate::records::module::Module;
use crate::records::source_module::SourceModule;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::{functions::freeze::freeze, functions::unfreeze::unfreeze};

impl Frontend {
    pub fn populate_expected_types(
        &self,
        source_module: &SourceModule,
        module: *mut Module,
        root_scope: &ScopePtr,
    ) {
        unsafe {
            let was_frozen = (*module).internal_types.types.is_frozen()
                || (*module).internal_types.type_packs.is_frozen();
            if was_frozen {
                unfreeze(&mut (*module).internal_types);
            }

            let mut visitor = ExpectedTypeVisitor::new(
                &mut (*module).ast_types,
                &mut (*module).ast_expected_types,
                &mut (*module).ast_resolved_types,
                &mut (*module).ast_overload_resolved_types,
                &mut (*module).internal_types,
                self.builtin_types,
                alloc::sync::Arc::as_ptr(root_scope) as *mut _,
            );

            luaur_ast::visit::ast_stat_block_visit(&*source_module.root, &mut visitor);

            if was_frozen {
                freeze(&mut (*module).internal_types);
            }
        }
    }
}
