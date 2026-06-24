use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::any_type::AnyType;
use crate::records::negation_type::NegationType;
use crate::records::never_type::NeverType;
use crate::records::recursion_limiter::RecursionLimiter;
use crate::records::table_type::TableType;
use crate::records::type_simplifier::TypeSimplifier;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl TypeSimplifier {
    pub fn simplify_type_id_dense_hash_set_type_id(
        &mut self,
        ty: TypeId,
        seen: &mut DenseHashSet<TypeId>,
    ) -> TypeId {
        let mut rl = RecursionLimiter {
            base: unsafe { core::mem::zeroed() },
            native_stack_guard: unsafe { core::mem::zeroed() },
        };
        rl.recursion_limiter_recursion_limiter(
            "TypeSimplifier::simplify",
            &mut self.recursion_depth,
            60,
        );

        let mut ty = unsafe { follow_type_id(ty) };
        if seen.contains(&ty) {
            return ty;
        }
        seen.insert(ty);

        if let Some(nt) = unsafe { get_type_id::<NegationType>(ty).as_ref() } {
            let negated_ty = unsafe { follow_type_id(nt.ty) };
            let bt = unsafe { &*self.builtin_types };
            if !unsafe { get_type_id::<AnyType>(negated_ty) }.is_null() {
                return unsafe { &mut *(self.arena as *mut crate::records::type_arena::TypeArena) }
                    .add_type(UnionType {
                        options: alloc::vec![bt.neverType, bt.errorType],
                    });
            } else if !unsafe { get_type_id::<UnknownType>(negated_ty) }.is_null() {
                return bt.neverType;
            } else if !unsafe { get_type_id::<NeverType>(negated_ty) }.is_null() {
                return bt.unknownType;
            }
            if let Some(nnt) = unsafe { get_type_id::<NegationType>(negated_ty).as_ref() } {
                return self.simplify_type_id_dense_hash_set_type_id(nnt.ty, seen);
            }
        }

        if let Some(tt) = unsafe { get_type_id::<TableType>(ty).as_ref() } {
            if tt.props.len() == 1 {
                if let Some(read_ty) = tt.props.values().next().unwrap().read_ty {
                    let prop_ty = self.simplify_type_id_dense_hash_set_type_id(read_ty, seen);
                    if !unsafe { get_type_id::<NeverType>(prop_ty) }.is_null() {
                        return unsafe { &*self.builtin_types }.neverType;
                    }
                }
            }
        }
        ty
    }
}
