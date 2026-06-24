use crate::records::normalization_too_complex::NormalizationTooComplex;
use crate::records::scope::Scope;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::type_checker_2::TypeChecker2;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;

impl TypeChecker2 {
    pub fn test_is_subtype_type_pack_id_type_pack_id_location(
        &mut self,
        sub_tp: TypePackId,
        super_tp: TypePackId,
        location: Location,
    ) -> bool {
        let scope: *mut Scope = self.find_innermost_scope(location);
        // C++: subtyping->isSubtype(subTy, superTy, scope, {}) — empty bindableGenerics.
        let empty_bindable: Vec<TypeId> = Vec::new();
        let mut r: SubtypingResult = unsafe {
            (*self.subtyping).is_subtype_type_pack_id_type_pack_id_not_null_scope_vector_type_id(
                sub_tp,
                super_tp,
                scope,
                &empty_bindable,
            )
        };

        if !self.is_error_suppressing_location_type_pack_id(location, sub_tp) {
            for e in &mut r.errors {
                e.location = location;
            }
        }
        self.report_errors(core::mem::take(&mut r.errors));

        if r.normalization_too_complex {
            self.report_error_type_error_data_location(
                TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
                &location,
            );
        }

        if !r.is_subtype {
            self.explain_error_type_pack_id_type_pack_id_location_subtyping_result(
                sub_tp, super_tp, location, &r,
            );
        }

        r.is_subtype
    }
}
