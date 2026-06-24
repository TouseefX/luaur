//! @interface-stub
use crate::enums::polarity::Polarity;
use crate::functions::fresh_index::fresh_index;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::type_checker::TypeChecker;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeChecker {
    pub fn fresh_type_pack_type_level(&mut self, level: TypeLevel) -> TypePackId {
        unsafe {
            let module = alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap())
                as *mut crate::records::module::Module;
            (*module).internal_types.add_type_pack_t(FreeTypePack {
                index: fresh_index(),
                level,
                scope: core::ptr::null_mut(),
                polarity: Polarity::None,
            })
        }
    }
}
