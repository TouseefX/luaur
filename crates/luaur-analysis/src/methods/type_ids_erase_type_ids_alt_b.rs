use crate::records::type_ids::TypeIds;
use crate::type_aliases::const_iterator::ConstIterator;
use crate::type_aliases::type_id::TypeId;

impl TypeIds {
    pub fn erase_type_id(&mut self, ty: TypeId) {
        let it = self.begin().into_iter().position(|x| x == ty).map(|pos| {
            let mut iter = self.begin().into_iter();
            for _ in 0..pos {
                iter.next();
            }
            iter
        });
        if let Some(mut it) = it {
            let _ = self.erase_type_ids_const_iterator(it);
        }
    }
}
