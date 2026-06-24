use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_optional::is_optional;
use crate::records::table_type::TableType;
use crate::records::unifier::Unifier;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use std::collections::HashMap;

impl Unifier {
    pub fn unifier_deeply_optional(
        &mut self,
        mut ty: TypeId,
        seen: &mut HashMap<TypeId, TypeId>,
    ) -> TypeId {
        ty = unsafe { follow_type_id(ty) };

        if is_optional(ty) {
            return ty;
        }

        // SAFETY: get_type_id returns a raw pointer that must be dereferenced
        let ttv_ptr = unsafe { get_type_id::<TableType>(ty) };

        if !ttv_ptr.is_null() {
            let ttv = unsafe { &*ttv_ptr };

            if let Some(&result) = seen.get(&ty) {
                return result;
            }

            // Add the table type to the arena first to get a result TypeId
            let result = unsafe { (*self.types).add_type(ttv.clone()) };

            // Store the result in seen map to handle cycles
            seen.insert(ty, result);

            // SAFETY: get_mutable_type_id returns a raw mutable pointer
            let result_ttv_ptr = unsafe { get_mutable_type_id::<TableType>(result) };
            LUAU_ASSERT!(!result_ttv_ptr.is_null());
            let result_ttv = unsafe { &mut *result_ttv_ptr };

            // Recursively make each property deeply optional
            for (_name, prop) in &mut result_ttv.props {
                let prop_ty = prop.read_ty.or(prop.write_ty).unwrap_or(core::ptr::null());
                let new_prop_ty = self.unifier_deeply_optional(prop_ty, seen);
                prop.read_ty = Some(new_prop_ty);
                prop.write_ty = Some(new_prop_ty);
            }

            // Return nil | result
            let builtin_types = unsafe { &*self.builtin_types };
            let union_types = alloc::vec![builtin_types.nilType, result];
            let union_type = UnionType {
                options: union_types,
            };
            unsafe { (*self.types).add_type(union_type) }
        } else {
            // Return nil | ty
            let builtin_types = unsafe { &*self.builtin_types };
            let union_types = alloc::vec![builtin_types.nilType, ty];
            let union_type = UnionType {
                options: union_types,
            };
            unsafe { (*self.types).add_type(union_type) }
        }
    }
}
