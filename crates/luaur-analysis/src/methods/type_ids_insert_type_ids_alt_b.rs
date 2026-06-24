use crate::functions::follow_type::follow_type_id;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::type_id::TypeId;

impl TypeIds {
    pub fn insert_type_id(&mut self, ty: TypeId) {
        let ty = unsafe { follow_type_id(ty) };

        // get a reference to the slot for `ty` in `types`
        let entry = self.types.get_or_insert(ty);

        // if `ty` is fresh, we can set it to `true`, add it to the order and hash and be done.
        if !*entry {
            *entry = true;
            self.order.push(ty);
            self.hash ^= ty as usize;
        }
    }
}
