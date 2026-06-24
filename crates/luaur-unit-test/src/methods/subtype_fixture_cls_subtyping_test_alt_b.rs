use crate::records::subtype_fixture::SubtypeFixture;
use luaur_analysis::functions::get_mutable_type::get_mutable;
use luaur_analysis::records::extern_type::ExternType;
use luaur_analysis::type_aliases::props_type_alt_c::Props;
use luaur_analysis::type_aliases::type_id::TypeId;

impl SubtypeFixture {
    pub fn cls_string_extern_type_props(&mut self, name: &str, props: Props) -> TypeId {
        let ty = self.cls_string_optional_type_id(name, None);
        let extern_ty = unsafe { get_mutable::<ExternType>(ty) };
        unsafe { (*extern_ty).props = props };
        ty
    }
}
