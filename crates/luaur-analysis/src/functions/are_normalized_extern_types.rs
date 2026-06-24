use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_subclass_type::is_subclass_extern_type_extern_type;
use crate::records::extern_type::ExternType;
use crate::records::normalized_extern_type::NormalizedExternType;
use crate::type_aliases::type_id::TypeId;

pub fn are_normalized_extern_types(tys: &NormalizedExternType) -> bool {
    let extern_types = &tys.extern_types;

    for (ty, negations) in extern_types.iter() {
        unsafe {
            if get_type_id::<ExternType>(*ty).is_null() {
                return false;
            }

            for &negation in &negations.order {
                if get_type_id::<ExternType>(negation).is_null() {
                    return false;
                }

                let etv = &*get_type_id::<ExternType>(*ty);
                let nctv = &*get_type_id::<ExternType>(negation);

                if !is_subclass_extern_type_extern_type(nctv, etv) {
                    return false;
                }
            }
        }

        for (other_ty, other_negations) in extern_types.iter() {
            if *other_ty == *ty {
                continue;
            }

            unsafe {
                if get_type_id::<ExternType>(*other_ty).is_null() {
                    return false;
                }

                let etv = &*get_type_id::<ExternType>(*ty);
                let octv = &*get_type_id::<ExternType>(*other_ty);

                if is_subclass_extern_type_extern_type(etv, octv) {
                    let iss = |t: TypeId| -> bool {
                        let c = unsafe { &*get_type_id::<ExternType>(t) };
                        is_subclass_extern_type_extern_type(etv, c)
                    };

                    if !other_negations.order.iter().any(|&t| iss(t)) {
                        return false;
                    }
                }
            }
        }
    }

    true
}
