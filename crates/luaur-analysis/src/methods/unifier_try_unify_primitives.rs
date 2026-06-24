use crate::functions::get_type_alt_j::get_type_id;
use crate::records::primitive_type::PrimitiveType;
use crate::records::type_mismatch::TypeMismatch;
use crate::records::unifier::Unifier;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;

impl Unifier {
    pub fn unifier_try_unify_primitives(&mut self, sub_ty: TypeId, super_ty: TypeId) {
        let super_prim = unsafe { get_type_id::<PrimitiveType>(super_ty) };
        let sub_prim = unsafe { get_type_id::<PrimitiveType>(sub_ty) };

        if super_prim.is_null() || sub_prim.is_null() {
            self.ice_string("passed non primitive types to unifyPrimitives");
            return;
        }

        if unsafe { (*super_prim).r#type != (*sub_prim).r#type } {
            let context = self.unifier_mismatch_context();
            self.report_error_location_type_error_data(
                self.location,
                TypeErrorData::TypeMismatch(TypeMismatch {
                    wanted_type: super_ty,
                    given_type: sub_ty,
                    reason: String::new(),
                    error: None,
                    context,
                }),
            );
        }
    }
}
