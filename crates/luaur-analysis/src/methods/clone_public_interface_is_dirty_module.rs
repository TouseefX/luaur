use crate::functions::get_type_alt_j::get_type_id;
use crate::records::clone_public_interface::ClonePublicInterface;
use crate::records::function_type::FunctionType;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;

impl ClonePublicInterface {
    /// `bool ClonePublicInterface::isDirty(TypeId ty)`.
    /// Reference: `Module.cpp:134-144`.
    pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
        let module = unsafe { &*self.module };

        let owning_arena = unsafe { (*ty).owning_arena };
        if owning_arena == (&module.internal_types as *const _ as *mut _) {
            return true;
        }

        let ftv = unsafe { get_type_id::<FunctionType>(ty) };
        if !ftv.is_null() {
            return unsafe { (*ftv).level.level } != 0;
        }

        let ttv = unsafe { get_type_id::<TableType>(ty) };
        if !ttv.is_null() {
            return unsafe { (*ttv).level.level } != 0;
        }

        false
    }
}
