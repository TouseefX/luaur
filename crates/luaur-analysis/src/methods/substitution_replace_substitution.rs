use crate::records::substitution::Substitution;
use crate::type_aliases::type_id::TypeId;

impl Substitution {
    pub fn replace_type_id(&mut self, ty: TypeId) -> TypeId {
        let ty = unsafe { (*self.base.log).follow_type_id(ty) };
        match self.new_types.find(&ty) {
            Some(prev_ty) => *prev_ty,
            None => ty,
        }
    }
}
