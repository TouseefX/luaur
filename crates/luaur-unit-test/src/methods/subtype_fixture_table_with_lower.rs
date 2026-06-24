use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::records::property_type::Property;
use luaur_analysis::type_aliases::type_id::TypeId;

impl SubtypeFixture {
    pub fn table_with_lower(&mut self) -> TypeId {
        let string_ty = self.builtin_types.stringType;
        let lower_ty = self.fn_item_initializer_list_type_id_initializer_list_type_id(
            vec![string_ty],
            vec![string_ty],
        );

        self.tbl(SubtypeFixture::props(vec![(
            "lower",
            Property::rw_type_id(lower_ty),
        )]))
    }
}
