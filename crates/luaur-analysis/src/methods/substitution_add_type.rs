use crate::records::substitution::Substitution;
use crate::type_aliases::type_id::TypeId;

impl Substitution {
    pub fn add_type<T>(&mut self, tv: T) -> TypeId
    where
        T: Into<crate::records::r#type::Type>,
    {
        unsafe { (*self.arena).add_tv(tv.into()) }
    }
}
