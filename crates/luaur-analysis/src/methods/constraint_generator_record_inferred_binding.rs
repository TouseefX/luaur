//! @interface-stub
use crate::records::constraint_generator::{ConstraintGenerator, InferredBinding};
use crate::records::symbol::Symbol;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_local::AstLocal;
use luaur_ast::records::location::Location;

impl luaur_common::records::dense_hash_table::DenseDefault for InferredBinding {
    fn dense_default() -> Self {
        Self {
            scope: core::ptr::null_mut(),
            location: Location::default(),
            types: TypeIds::type_ids(),
        }
    }
}

impl ConstraintGenerator {
    pub fn record_inferred_binding(&mut self, local: *mut AstLocal, ty: TypeId) {
        if let Some(ib) = self.inferred_bindings.find_mut(&Symbol::from_local(local)) {
            ib.types.insert_type_id(ty);
        }
    }
}
