use crate::enums::variance::Variance;
use crate::functions::get_singleton_type::get_singleton_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::type_mismatch::TypeMismatch;
use crate::records::unifier::Unifier;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;

impl Unifier {
    pub fn unifier_try_unify_singletons(&mut self, sub_ty: TypeId, super_ty: TypeId) {
        let super_prim = unsafe { get_type_id::<PrimitiveType>(super_ty) };
        let super_singleton = unsafe { get_type_id::<SingletonType>(super_ty) };
        let sub_singleton = unsafe { get_type_id::<SingletonType>(sub_ty) };

        if (super_prim.is_null() && super_singleton.is_null()) || sub_singleton.is_null() {
            self.ice_string("passed non singleton/primitive types to unifySingletons");
            return;
        }

        if !super_singleton.is_null() && unsafe { *super_singleton == *sub_singleton } {
            return;
        }

        if !super_prim.is_null()
            && unsafe { (*super_prim).r#type == PrimitiveType::Boolean }
            && !get_singleton_type::<BooleanSingleton>(sub_singleton).is_null()
            && self.variance == Variance::Covariant
        {
            return;
        }

        if !super_prim.is_null()
            && unsafe { (*super_prim).r#type == PrimitiveType::String }
            && !get_singleton_type::<StringSingleton>(sub_singleton).is_null()
            && self.variance == Variance::Covariant
        {
            return;
        }

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
