use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::records::property_type::Property;

#[cfg(test)]
#[test]
fn subtyping_x_string_write_x_string() {
    let mut fixture = SubtypeFixture::default();
    let string_ty = fixture.builtin_types.stringType;
    let left = fixture.tbl(SubtypeFixture::props(vec![(
        "x",
        Property::rw_type_id(string_ty),
    )]));
    let right = fixture.tbl(SubtypeFixture::props(vec![(
        "x",
        Property::writeonly(string_ty),
    )]));

    assert!(fixture.is_subtype_type_id_type_id(left, right).is_subtype());
}
