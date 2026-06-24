use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::records::property_type::Property;

#[cfg(test)]
#[test]
fn subtyping_t_x_t_t_method_t_x_t_t_x_number_number() {
    let mut fixture = SubtypeFixture::default();
    let generic_t = fixture.generic("T");
    let number_ty = fixture.builtin_types.numberType;

    let table_to_prop_type = {
        let table_with_x = fixture.tbl(SubtypeFixture::props(vec![(
            "x",
            Property::rw_type_id(generic_t),
        )]));
        fixture.generic_fn(vec![generic_t], vec![table_with_x], vec![generic_t])
    };

    let table_with_method_and_x = fixture.tbl(SubtypeFixture::props(vec![
        ("method", Property::rw_type_id(table_to_prop_type)),
        ("x", Property::rw_type_id(number_ty)),
    ]));
    let other_type = fixture.fn_item_initializer_list_type_id_initializer_list_type_id(
        vec![table_with_method_and_x],
        vec![number_ty],
    );

    assert!(fixture
        .is_subtype_type_id_type_id(table_to_prop_type, other_type)
        .is_subtype());
}
