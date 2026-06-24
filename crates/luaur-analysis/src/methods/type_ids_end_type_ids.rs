use crate::records::type_ids::TypeIds;
use crate::type_aliases::iterator_type_ids::iterator;

impl TypeIds {
    pub fn end_mut(&mut self) -> iterator {
        // iterator = IterMut<'static, TypeId> (via the type alias) cannot soundly
        // be produced from a borrowed &mut self. Delegate to the existing wrapper
        // method on TypeIds that already provides the correct return behavior.
        self.end_mut()
    }
}
