//! @interface-stub
use crate::enums::table_state::TableState;
use crate::functions::follow_type::follow;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::metatable_type::MetatableType;
use crate::records::table_type::TableType;
use crate::records::type_ids::TypeIds;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use alloc::collections::VecDeque;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl luaur_common::records::dense_hash_table::DenseDefault for TypeIds {
    fn dense_default() -> Self {
        TypeIds::type_ids()
    }
}

impl ConstraintGenerator {
    pub fn record_property_assignment(&mut self, ty: TypeId) -> bool {
        let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(core::ptr::null());
        let mut queue = VecDeque::new();

        queue.push_back(ty);

        let mut incremented = false;

        while let Some(front) = queue.pop_front() {
            let t = unsafe { follow(front) };

            if seen.find(&t).is_some() {
                continue;
            }

            seen.insert(t);

            unsafe {
                let tt = get_mutable_type_id::<TableType>(t);
                if !tt.is_null() && (*tt).state == TableState::Unsealed {
                    (*tt).remaining_props += 1;
                    incremented = true;
                    continue;
                }

                let mt = get_type_id::<MetatableType>(t);
                if !mt.is_null() {
                    queue.push_back((*mt).table);
                    continue;
                }

                if let Some(local_domain) = self.local_types.find(&t) {
                    for &domain_ty in &local_domain.order {
                        queue.push_back(domain_ty);
                    }
                    continue;
                }

                let ut = get_type_id::<UnionType>(t);
                if !ut.is_null() {
                    for &part in &(*ut).options {
                        queue.push_back(part);
                    }
                }
            }
        }

        incremented
    }
}
