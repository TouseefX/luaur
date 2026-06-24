use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::records::property_type::Property;

#[cfg(test)]
#[test]
fn subtyping_metatable_x_number_x_number() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let left = fixture.meta(
        SubtypeFixture::props(vec![("x", Property::rw_type_id(number_ty))]),
        SubtypeFixture::props(vec![]),
    );
    let right = fixture.tbl(SubtypeFixture::props(vec![(
        "x",
        Property::rw_type_id(number_ty),
    )]));

    assert!(!fixture.is_subtype_type_id_type_id(left, right).is_subtype());
}
