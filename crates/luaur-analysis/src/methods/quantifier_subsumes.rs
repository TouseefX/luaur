use crate::records::quantifier::Quantifier;
use crate::records::scope::Scope;
use alloc::sync::Arc;

impl Quantifier {
    pub fn subsumes(&mut self, outer: *mut Scope, inner: *mut Scope) -> bool {
        let mut current = inner;
        while !current.is_null() {
            if current == outer {
                return true;
            }
            current = unsafe { (*current).parent.as_ref() }
                .map_or(std::ptr::null_mut(), |sp| Arc::as_ptr(sp) as *mut Scope);
        }
        false
    }
}
