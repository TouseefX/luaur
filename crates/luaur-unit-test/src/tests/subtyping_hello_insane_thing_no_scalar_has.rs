use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_hello_insane_thing_no_scalar_has() {
    let mut fixture = SubtypeFixture::default();
    let hello_ty = fixture.str("hello");
    let table_without_scalar_prop = fixture.table_without_scalar_prop();

    assert!(!fixture
        .is_subtype_type_id_type_id(hello_ty, table_without_scalar_prop)
        .is_subtype());
}
