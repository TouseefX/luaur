use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_string_insane_thing_no_scalar_has() {
    let mut fixture = SubtypeFixture::default();
    let string_ty = fixture.builtin_types.stringType;
    let table_without_scalar_prop = fixture.table_without_scalar_prop();

    assert!(!fixture
        .is_subtype_type_id_type_id(string_ty, table_without_scalar_prop)
        .is_subtype());
}
