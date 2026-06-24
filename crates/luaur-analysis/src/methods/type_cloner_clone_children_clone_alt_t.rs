use crate::records::lazy_type::LazyType;
use crate::records::type_cloner::TypeCloner;

impl TypeCloner {
    pub fn clone_children_lazy_type(&mut self, t: *mut LazyType) {
        // The `FragmentAutocompleteTypeCloner` override (Clone.cpp:541-544) does
        // not clone lazy types: it overrides `cloneChildren(LazyType*)` to a no-op.
        if self.skip_lazy_type_clone {
            return;
        }
        unsafe {
            if let Some(unwrapped) = (*t).unwrapped.as_ref() {
                (*t).unwrapped = self.shallow_clone_type_id(unwrapped as *const _ as *mut _);
            }
        }
    }
}
