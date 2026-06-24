use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::reduce_union::reduce_union;
use crate::records::free_type::FreeType;
use crate::records::type_checker::TypeChecker;
use crate::records::union_type::UnionType;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl TypeChecker {
    pub fn union_of_types(
        &mut self,
        a: TypeId,
        b: TypeId,
        scope: &ScopePtr,
        location: &Location,
        unify_free_types: bool,
    ) -> TypeId {
        let mut a = unsafe { follow_type_id(a) };
        let mut b = unsafe { follow_type_id(b) };

        if unify_free_types {
            let is_a_free = unsafe { !get_type_id::<FreeType>(a).is_null() };
            let is_b_free = unsafe { !get_type_id::<FreeType>(b).is_null() };

            if is_a_free || is_b_free {
                if self.unify_type_id_type_id_scope_ptr_location(b, a, scope, location) {
                    return a;
                }

                return self.error_recovery_type_type_id(self.any_type);
            }
        }

        if a == b {
            return a;
        }

        let types = reduce_union(&[a, b]);
        if types.is_empty() {
            return self.never_type;
        }

        if types.len() == 1 {
            return types[0];
        }

        self.add_type(&UnionType { options: types })
    }
}
