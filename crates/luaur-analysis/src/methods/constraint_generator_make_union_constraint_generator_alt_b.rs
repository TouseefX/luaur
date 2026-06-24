//! @interface-stub
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::union_builder::UnionBuilder;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

impl ConstraintGenerator {
    pub fn make_union_vector_type_id(&mut self, options: alloc::vec::Vec<TypeId>) -> TypeId {
        let mut ub = UnionBuilder::union_builder(self.arena, self.builtin_types);
        ub.reserve(options.len());

        for option in options {
            ub.add(option);
        }

        let union_ty = ub.build();

        unsafe {
            if !get_type_id::<UnionType>(union_ty).is_null() {
                self.unions_to_simplify.push(union_ty);
            }
        }

        union_ty
    }
}
