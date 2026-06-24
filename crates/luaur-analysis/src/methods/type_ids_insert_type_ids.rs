use crate::records::iterator::Iterator;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::variant::Variant3;

impl TypeIds {
    pub fn insert_iterator_iterator(&mut self, _begin: Iterator, _end: Iterator) {
        let mut it = _begin;
        loop {
            if it.operator_eq(&_end) {
                break;
            }
            let ty_variant: Variant3<
                TypeId,
                *const crate::records::type_pack_var::TypePackVar,
                *const crate::records::constraint::Constraint,
            > = it.operator_deref();
            let ty = match ty_variant {
                Variant3::V0(ty) => ty,
                _ => continue,
            };
            self.insert_type_id(ty);
            it.operator_inc();
        }
    }
}
