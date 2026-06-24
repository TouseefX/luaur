use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::records::property_type::Property;

#[cfg(test)]
#[test]
fn subtyping_vec_2_x_number_y_number() {
    let mut fixture = SubtypeFixture::default();
    let vec2_class = fixture.vec2_class();
    let number_ty = fixture.builtin_types.numberType;
    let xy = fixture.tbl(SubtypeFixture::props(vec![
        ("X", Property::rw_type_id(number_ty)),
        ("Y", Property::rw_type_id(number_ty)),
    ]));

    assert!(fixture
        .is_subtype_type_id_type_id(vec2_class, xy)
        .is_subtype());
}
