//! Source: `Analysis/src/TypePath.cpp:393-467` (hand-ported)
use crate::functions::begin_type_pack::begin;
use crate::functions::end_type_pack::end;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_or_pack::get_type_or_pack_mut as get_type_or_pack;
use crate::functions::get_type_or_pack_alt_s::get_type_or_pack_mut_2;
use crate::records::index::Index;
use crate::records::intersection_type::IntersectionType;
use crate::records::traversal_state::TraversalState;
use crate::records::type_iterator::TypeIterator;
use crate::records::type_pack::TypePack;
use crate::records::union_type::UnionType;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TraversalState {
    pub fn traverse_type_path_index(&mut self, index: &Index) -> bool {
        if self.check_invariants() {
            return false;
        }

        let current_type = unsafe { get_type_or_pack::<TypeId>(&self.current) };
        if !current_type.is_null() {
            let current_type_id = unsafe { follow_type_id(*current_type) };
            let mut updated_current = false;

            if !unsafe { get_type_id::<ErrorType>(current_type_id) }.is_null() {
                self.encountered_error_suppression = true;
                return false;
            }

            let u = unsafe { get_type_id::<UnionType>(current_type_id) };
            if !u.is_null() {
                // We want to track the index that updates the current type with
                // `idx` while still iterating through the entire union to check
                // for error types.
                let mut idx: usize = 0;
                let mut it = TypeIterator::<UnionType>::type_iterator_type(u);
                let end_it = TypeIterator::<UnionType>::type_iterator_default();
                while it.operator_ne(&end_it) {
                    let opt_ty = it.operator_deref();
                    it.operator_inc();

                    if !unsafe { get_type_id::<ErrorType>(opt_ty) }.is_null() {
                        self.encountered_error_suppression = true;
                    }
                    if idx == index.index {
                        self.update_current_type_id(opt_ty);
                        updated_current = true;
                    }
                    idx += 1;
                }
            } else {
                let i = unsafe { get_type_id::<IntersectionType>(current_type_id) };
                if !i.is_null() {
                    let mut idx: usize = 0;
                    let mut it = TypeIterator::<IntersectionType>::type_iterator_type(i);
                    let end_it = TypeIterator::<IntersectionType>::type_iterator_default();
                    while it.operator_ne(&end_it) {
                        let part_ty = it.operator_deref();
                        it.operator_inc();

                        if !unsafe { get_type_id::<ErrorType>(part_ty) }.is_null() {
                            self.encountered_error_suppression = true;
                        }
                        if idx == index.index {
                            self.update_current_type_id(part_ty);
                            updated_current = true;
                        }
                        idx += 1;
                    }
                }
            }

            updated_current
        } else {
            let current_pack = unsafe { get_type_or_pack::<TypePackId>(&self.current) };
            LUAU_ASSERT!(!current_pack.is_null());
            if !unsafe { get_type_or_pack_mut_2::<TypePack>(&self.current) }.is_null() {
                let cp: TypePackId = unsafe { *current_pack };
                let mut it = begin(cp);
                let mut i: usize = 0;
                while i < index.index && it.operator_ne(&end(cp)) {
                    it.operator_inc();
                    i += 1;
                }

                if it.operator_ne(&end(cp)) {
                    self.update_current_type_id(*it.operator_deref());
                    return true;
                }
            }

            false
        }
    }
}
