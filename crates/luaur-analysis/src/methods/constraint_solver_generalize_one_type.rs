use crate::functions::follow_type::follow_type_id;
use crate::functions::generalize::generalize;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::to_string_to_string_alt_b::to_string_type_id_to_string_options_mut;
use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::free_type::FreeType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::FFlag;

impl ConstraintSolver {
    pub fn generalize_one_type(&mut self, ty: TypeId) {
        let ty = unsafe { follow_type_id(ty) };
        let free_ty = unsafe { get_type_id::<FreeType>(ty) };

        let saveme = if FFlag::DebugLuauLogSolver.get() {
            to_string_type_id_to_string_options_mut(ty, self.opts.clone())
        } else {
            "[FFlag::DebugLuauLogSolver Off]".to_string()
        };

        if free_ty.is_null() {
            return;
        }

        let free_ty_ref = unsafe { &*free_ty };
        let function_type = self
            .constraint_set
            .scope_to_function
            .find(&free_ty_ref.scope);

        if let Some(function_type) = function_type {
            let result_ty = generalize(
                self.arena,
                self.builtin_types,
                free_ty_ref.scope,
                &self.generalized_types_ as *const _ as *mut _,
                *function_type,
                Some(ty),
            );

            if FFlag::DebugLuauLogSolver.get() {
                let current_ty_str = to_string_type_id(ty);
                let result_ty_str = result_ty
                    .map(|t| to_string_type_id(t))
                    .unwrap_or_else(|| to_string_type_id(*function_type));

                println!(
                    "Eagerly generalized {} (now {})\n\tin function {}",
                    saveme, current_ty_str, result_ty_str
                );
            }
        }
    }
}
