use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_nil::is_nil;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariantMember;

#[allow(clippy::missing_safety_doc)]
pub unsafe fn return_first_nonnull_option_of_type<T: TypeVariantMember + 'static>(
    utv: &UnionType,
) -> Option<*const T> {
    let mut ret: Option<*const T> = None;

    for &sub_ty in &utv.options {
        if is_nil(sub_ty) {
            continue;
        }

        let follow_ty: TypeId = follow_type_id(sub_ty);
        let ftv: *const T = get_type_id::<T>(follow_ty);

        if ftv.is_null() {
            return None;
        }

        if ret.is_some() {
            return None;
        }

        ret = Some(ftv);
    }

    ret
}
