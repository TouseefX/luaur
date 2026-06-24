//! @interface-stub
use crate::functions::get_mutable_txn_log::get_mutable_pending_type;
use crate::functions::get_mutable_txn_log_alt_c::get_mutable_pending_type_pack;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::records::txn_log::TxnLog;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariantMember;
use crate::type_aliases::type_variant::TypeVariantMember;

pub trait TxnLogGetMutable<TID>: Sized {
    unsafe fn get_mutable_from_log(log: &TxnLog, ty: TID) -> *mut Self;
}

impl<T: TypeVariantMember + 'static> TxnLogGetMutable<TypeId> for T {
    unsafe fn get_mutable_from_log(log: &TxnLog, ty: TypeId) -> *mut Self {
        let pending_ty = log.pending_type_id(ty);
        if !pending_ty.is_null() {
            return get_mutable_pending_type::<T>(pending_ty);
        }

        get_mutable_type_id::<T>(ty)
    }
}

impl<T: TypePackVariantMember + 'static> TxnLogGetMutable<TypePackId> for T {
    unsafe fn get_mutable_from_log(log: &TxnLog, tp: TypePackId) -> *mut Self {
        let pending_tp = log.pending_type_pack_id(tp);
        if !pending_tp.is_null() {
            return get_mutable_pending_type_pack::<T>(pending_tp);
        }

        get_mutable_type_pack_id::<T>(tp)
    }
}

impl TxnLog {
    pub fn txn_log_get_mutable<T, TID>(&self, ty: TID) -> *mut T
    where
        T: TxnLogGetMutable<TID>,
    {
        unsafe { T::get_mutable_from_log(self, ty) }
    }
}
