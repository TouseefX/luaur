use crate::records::fixture::Fixture;
use luaur_analysis::functions::follow_type::follow_type_id;
use luaur_analysis::functions::get_type_alt_j::get_type_id;
use luaur_analysis::records::primitive_type::{PrimitiveType, Type};
use luaur_analysis::type_aliases::type_id::TypeId;

impl Fixture {
    pub fn get_primitive_type(&mut self, ty: TypeId) -> Option<Type> {
        luaur_common::LUAU_ASSERT!(!ty.is_null());

        let a_type = unsafe { follow_type_id(ty) };
        luaur_common::LUAU_ASSERT!(!a_type.is_null());

        let pt_ptr = unsafe { get_type_id::<PrimitiveType>(a_type) };
        if !pt_ptr.is_null() {
            let pt = unsafe { &*pt_ptr };
            Some(pt.r#type)
        } else {
            None
        }
    }
}
