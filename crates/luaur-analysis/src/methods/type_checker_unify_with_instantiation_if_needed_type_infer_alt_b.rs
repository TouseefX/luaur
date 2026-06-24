use crate::functions::is_generic::is_generic;
use crate::functions::maybe_generic::maybe_generic;
use crate::records::type_checker::TypeChecker;
use crate::records::unifier::Unifier;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

impl TypeChecker {
    pub fn unify_with_instantiation_if_needed_type_id_type_id_scope_ptr_unifier(
        &mut self,
        sub_ty: TypeId,
        super_ty: TypeId,
        scope: ScopePtr,
        state: &mut Unifier,
    ) {
        LUAU_ASSERT!(!FFlag::LuauInstantiateInSubtyping.get());

        if !maybe_generic(sub_ty) {
            state.try_unify_type_id_type_id_bool_bool_literal_properties(
                sub_ty, super_ty, false, false, None,
            );
        } else if !maybe_generic(super_ty) && is_generic(sub_ty) {
            let instantiated = self.instantiate(&scope, sub_ty, state.location, core::ptr::null());
            state.try_unify_type_id_type_id_bool_bool_literal_properties(
                instantiated,
                super_ty,
                false,
                false,
                None,
            );
        } else {
            let mut child = state.unifier_make_child_unifier();
            child.try_unify_type_id_type_id_bool_bool_literal_properties(
                sub_ty, super_ty, false, false, None,
            );

            if !child.errors.is_empty() {
                let instantiated =
                    self.instantiate(&scope, sub_ty, state.location, &child.log as *const _);

                if sub_ty == instantiated {
                    state.log.concat(child.log);
                    state.errors.append(&mut child.errors);
                } else {
                    state.try_unify_type_id_type_id_bool_bool_literal_properties(
                        instantiated,
                        super_ty,
                        false,
                        false,
                        None,
                    );
                }
            } else {
                state.log.concat(child.log);
            }
        }
    }
}
