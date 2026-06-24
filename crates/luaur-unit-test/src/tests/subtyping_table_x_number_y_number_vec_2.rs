use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::records::property_type::Property;

#[cfg(test)]
#[test]
fn subtyping_table_x_number_y_number_vec_2() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let table_ty = fixture.builtin_types.tableType;
    let x = fixture.tbl(SubtypeFixture::props(vec![
        ("X", Property::rw_type_id(number_ty)),
        ("Y", Property::rw_type_id(number_ty)),
    ]));
    let left = fixture.meet(table_ty, x);
    let vec2_class = fixture.vec2_class();

    assert!(!fixture
        .is_subtype_type_id_type_id(left, vec2_class)
        .is_subtype());
}
