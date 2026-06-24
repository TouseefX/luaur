use crate::records::type_pack_function::TypePackFunction;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

#[derive(Debug, Clone)]
pub struct TypeFunctionInstanceTypePack {
    pub(crate) function: *const TypePackFunction,
    pub(crate) typeArguments: alloc::vec::Vec<TypeId>,
    pub(crate) packArguments: alloc::vec::Vec<TypePackId>,
}

impl Drop for TypeFunctionInstanceTypePack {
    fn drop(&mut self) {
        unsafe {
            core::ptr::write(&mut self.typeArguments, alloc::vec::Vec::new());
            core::ptr::write(&mut self.packArguments, alloc::vec::Vec::new());
        }
    }
}
