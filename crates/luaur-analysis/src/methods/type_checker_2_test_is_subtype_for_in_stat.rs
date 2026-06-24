use crate::records::function_type::FunctionType;
use crate::records::normalization_too_complex::NormalizationTooComplex;
use crate::records::type_checker_2::TypeChecker2;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_stat_for_in::AstStatForIn;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeChecker2 {
    pub fn test_is_subtype_for_in_stat(
        &mut self,
        iter_func: TypeId,
        prospective_func: TypeId,
        for_in_stat: *const AstStatForIn,
    ) {
        unsafe {
            LUAU_ASSERT!(
                !crate::functions::get_type_alt_j::get_type_id::<FunctionType>(
                    crate::functions::follow_type::follow_type_id(iter_func)
                )
                .is_null()
            );
            LUAU_ASSERT!(
                !crate::functions::get_type_alt_j::get_type_id::<FunctionType>(
                    crate::functions::follow_type::follow_type_id(prospective_func)
                )
                .is_null()
            );

            let iter_func_location = (*(*for_in_stat).values.data.add(0).read()).base.location;

            let scope = self.find_innermost_scope(iter_func_location);
            let mut r = (*self.subtyping).is_subtype_type_id_type_id_not_null_scope(
                iter_func,
                prospective_func,
                scope,
            );

            if !self.is_error_suppressing_location_type_id(iter_func_location, iter_func) {
                for e in &mut r.errors {
                    e.location = iter_func_location;
                }
            }

            self.report_errors(core::mem::take(&mut r.errors));

            if r.normalization_too_complex {
                self.report_error_type_error_data_location(
                    NormalizationTooComplex::default().into(),
                    &iter_func_location,
                );
            }

            if r.is_subtype {
                return;
            }

            self.explain_error_type_id_type_id_location_subtyping_result(
                iter_func,
                prospective_func,
                iter_func_location,
                &r,
            );
        }
    }
}
