//! @interface-stub
use crate::functions::follow_type::follow;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::simplify_union::simplify_union;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::never_type::NeverType;
use crate::records::scope::Scope;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl ConstraintGenerator {
    pub fn make_union_scope_ptr_location_type_id_type_id(
        &mut self,
        _scope: *mut Scope,
        _location: Location,
        lhs: TypeId,
        rhs: TypeId,
    ) -> TypeId {
        unsafe {
            if !get_type_id::<NeverType>(follow(lhs)).is_null() {
                return rhs;
            }

            if !get_type_id::<NeverType>(follow(rhs)).is_null() {
                return lhs;
            }

            let result = simplify_union(self.builtin_types, self.arena, lhs, rhs).result;

            if !get_type_id::<UnionType>(follow(result)).is_null() {
                self.unions_to_simplify.push(result);
            }

            result
        }
    }
}
