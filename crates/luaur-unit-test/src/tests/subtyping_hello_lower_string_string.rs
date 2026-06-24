use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_hello_lower_string_string() {
    let mut fixture = SubtypeFixture::default();
    let hello_ty = fixture.str("hello");
    let table_with_lower = fixture.table_with_lower();

    assert!(fixture
        .is_subtype_type_id_type_id(hello_ty, table_with_lower)
        .is_subtype());
}
