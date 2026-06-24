use crate::records::subtype_fixture::SubtypeFixture;
use luaur_analysis::records::property_type::Property;

#[cfg(test)]
#[test]
fn subtyping_t_1_where_t_1_trim_t_1_t_1_t_2_where_t_2_trim_t_2_string() {
    let mut fixture = SubtypeFixture::default();
    let string_ty = fixture.builtin_types.stringType;
    let t1 = fixture.cyclic_table(|fixture, ty, table| {
        let trim_ty =
            fixture.fn_item_initializer_list_type_id_initializer_list_type_id(vec![ty], vec![ty]);
        table
            .props
            .insert("trim".into(), Property::rw_type_id(trim_ty));
    });
    let t2 = fixture.cyclic_table(|fixture, ty, table| {
        let trim_ty = fixture
            .fn_item_initializer_list_type_id_initializer_list_type_id(vec![ty], vec![string_ty]);
        table
            .props
            .insert("trim".into(), Property::rw_type_id(trim_ty));
    });

    assert!(!fixture.is_subtype_type_id_type_id(t1, t2).is_subtype());
}
