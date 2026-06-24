use crate::records::def::Def;
use crate::records::non_strict_context::NonStrictContext;
use crate::type_aliases::type_id::TypeId;

impl NonStrictContext {
    pub fn find_def(&self, d: *const Def) -> Option<TypeId> {
        self.context.get(&d).copied()
    }
}
