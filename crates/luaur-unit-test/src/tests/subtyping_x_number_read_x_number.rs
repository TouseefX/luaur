use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::records::property_type::Property;

#[cfg(test)]
#[test]
fn subtyping_x_number_read_x_number() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let left = fixture.tbl(SubtypeFixture::props(vec![(
        "x",
        Property::rw_type_id(number_ty),
    )]));
    let right = fixture.tbl(SubtypeFixture::props(vec![(
        "x",
        Property::readonly(number_ty),
    )]));

    assert!(fixture.is_subtype_type_id_type_id(left, right).is_subtype());
}
