use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::constraint_set::ConstraintSet;
use crate::records::type_ids::TypeIds;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl ConstraintGenerator {
    pub fn run(&mut self, block: *mut AstStatBlock) -> ConstraintSet {
        self.visit_module_root(block);

        ConstraintSet {
            root_scope: self.root_scope,
            constraints: core::mem::take(&mut self.constraints),
            free_types: core::mem::replace(&mut self.free_types, TypeIds::type_ids()),
            scope_to_function: core::mem::replace(
                &mut self.scope_to_function,
                DenseHashMap::new(core::ptr::null_mut()),
            ),
            errors: core::mem::take(&mut self.errors),
        }
    }
}
