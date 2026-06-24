use crate::records::apply_type_function::ApplyTypeFunction;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ApplyTypeFunction {
    pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
        let arg = self.type_arguments.find(&ty).unwrap();
        LUAU_ASSERT!(!arg.is_null());
        *arg
    }
}
