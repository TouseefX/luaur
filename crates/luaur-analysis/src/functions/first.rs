use crate::functions::begin_type_pack::begin_type_pack_id;
use crate::functions::end_type_pack::end_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::type_pack_iterator::TypePackIterator;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

pub fn first(tp: TypePackId, ignore_hidden_variadics: bool) -> Option<TypeId> {
    let mut iter = begin_type_pack_id(tp);
    let end_iter = end_type_pack_id(tp);

    if iter.operator_ne(&end_iter) {
        return Some(*iter.operator_deref());
    }

    if let Some(tail) = iter.tail() {
        let vtp = unsafe { get_type_pack_id::<VariadicTypePack>(tail) };
        if !vtp.is_null() {
            let vtp_ref = unsafe { &*vtp };
            if !vtp_ref.hidden || !ignore_hidden_variadics {
                return Some(vtp_ref.ty);
            }
        }
    }

    None
}
