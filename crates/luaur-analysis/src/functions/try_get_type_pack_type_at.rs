use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

pub fn try_get_type_pack_type_at(tp: TypePackId, index: usize) -> Option<TypeId> {
    let (tp_head, tp_tail) = flatten_type_pack_id(tp);

    if index < tp_head.len() {
        return Some(tp_head[index as usize]);
    }

    if let Some(tp_tail_id) = tp_tail {
        let follow_tp = unsafe { follow_type_pack_id(tp_tail_id) };
        let vtp = unsafe { get_type_pack_id::<VariadicTypePack>(follow_tp) };
        if !vtp.is_null() {
            let vtp_ref = unsafe { &*vtp };
            return Some(vtp_ref.ty);
        }
    }

    None
}
