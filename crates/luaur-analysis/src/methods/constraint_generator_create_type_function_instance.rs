//! @interface-stub
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::reduce_constraint::ReduceConstraint;
use crate::records::type_function::TypeFunction;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;

impl ConstraintGenerator {
    pub fn create_type_function_instance(
        &mut self,
        function: &TypeFunction,
        type_arguments: Vec<TypeId>,
        pack_arguments: Vec<TypePackId>,
        scope: &ScopePtr,
        location: Location,
    ) -> TypeId {
        let result = unsafe {
            (*self.arena).add_type_function_type_function_vector_type_id_vector_type_pack_id(
                function,
                type_arguments,
                pack_arguments,
            )
        };

        self.add_constraint_scope_ptr_location_constraint_v(
            scope,
            location,
            ConstraintV::Reduce(ReduceConstraint { ty: result }),
        );

        result
    }
}
