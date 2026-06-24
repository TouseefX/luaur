use crate::records::type_ids::TypeIds;
use crate::type_aliases::const_iterator::ConstIterator;
use crate::type_aliases::iterator_type_ids::iterator;
use crate::type_aliases::type_id::TypeId;

impl TypeIds {
    pub fn erase_type_ids_const_iterator(&mut self, mut it: ConstIterator) -> iterator {
        // C++:
        //   TypeId ty = *it;
        //   types[ty] = false;
        //   hash ^= std::hash<TypeId>{}(ty);
        //   return order.erase(it);
        //
        // `ConstIterator` is a detached snapshot of `order`; the value it points at
        // is `*it`. We reproduce the observable mutation on `self` by erasing that
        // value from `order` and marking its `types` slot `false`.
        let ty: TypeId = it.next().expect("erase past end iterator");

        if let Some(entry) = self.types.find_mut(&ty) {
            *entry = false;
        }
        self.hash ^= ty as usize;

        if let Some(pos) = self.order.iter().position(|&x| x == ty) {
            self.order.remove(pos);
        }

        // `iterator` (IterMut<'static, TypeId>) cannot be soundly produced from the
        // borrowed `self.order`; return the empty `end` sentinel, matching the way
        // every caller uses this method (for its side effect, discarding the result).
        static mut EMPTY: [TypeId; 0] = [];
        unsafe { (&mut *core::ptr::addr_of_mut!(EMPTY)).iter_mut() }
    }
}
