use crate::enums::polarity::Polarity;
use crate::functions::fresh_type::fresh_type;
use crate::records::scope::Scope;
use crate::records::unifier_2::Unifier2;
use crate::type_aliases::type_id::TypeId;
use core::ptr::NonNull;

impl Unifier2 {
    pub fn fresh_type(&mut self, scope: NonNull<Scope>, polarity: Polarity) -> TypeId {
        let result = unsafe {
            fresh_type(
                &mut *self.arena.as_ptr(),
                &*self.builtin_types.as_ptr(),
                scope.as_ptr(),
                polarity,
            )
        };
        self.new_fresh_types.push(result);
        result
    }
}
