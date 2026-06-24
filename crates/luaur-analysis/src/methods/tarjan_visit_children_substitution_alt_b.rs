use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::tarjan::Tarjan;
use crate::records::type_pack::TypePack;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Tarjan {
    pub fn visit_children_type_pack_id_i32(&mut self, tp: TypePackId, _index: i32) {
        let mut tp = tp;
        unsafe {
            LUAU_ASSERT!(tp == (*self.log).follow_type_pack_id(tp));
        }

        if self.ignore_children_visit_type_pack_id(tp) {
            return;
        }

        let ptp = unsafe { (*self.log).pending_type_pack_id(tp) };
        if !ptp.is_null() {
            tp = unsafe { &(*ptp).pending as *const crate::records::type_pack_var::TypePackVar };
        }

        let tpp = unsafe { get_type_pack_id::<TypePack>(tp) };
        if !tpp.is_null() {
            let tpp = unsafe { &*tpp };
            for tv in tpp.head.iter() {
                self.visit_child_type_id(*tv);
            }
            if let Some(tail) = tpp.tail {
                self.visit_child_type_pack_id(tail);
            }
            return;
        }

        let vtp = unsafe { get_type_pack_id::<VariadicTypePack>(tp) };
        if !vtp.is_null() {
            let vtp = unsafe { &*vtp };
            self.visit_child_type_id(vtp.ty);
        }
    }
}
