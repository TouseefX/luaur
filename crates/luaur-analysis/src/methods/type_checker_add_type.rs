use crate::records::module::Module;
use crate::records::r#type::Type;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::type_id::TypeId;
use alloc::sync::Arc;

impl TypeChecker {
    pub fn add_type<T>(&mut self, tv: &T) -> TypeId
    where
        T: Clone + Into<Type> + 'static,
    {
        unsafe {
            let module =
                Arc::as_ptr(self.current_module.as_ref().expect("current_module")) as *mut Module;
            (*module).internal_types.add_type(tv.clone())
        }
    }
}
