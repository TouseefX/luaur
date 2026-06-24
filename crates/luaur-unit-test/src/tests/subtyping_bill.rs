use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::records::property_type::Property;

#[cfg(test)]
#[test]
fn subtyping_bill() {
    let mut fixture = SubtypeFixture::default();
    let string_ty = fixture.builtin_types.stringType;
    let number_ty = fixture.builtin_types.numberType;

    let a = fixture.tbl_with_indexer(
        SubtypeFixture::props(vec![("a", Property::rw_type_id(string_ty))]),
        string_ty,
        number_ty,
    );
    let b = fixture.tbl_with_indexer(
        SubtypeFixture::props(vec![("a", Property::rw_type_id(string_ty))]),
        string_ty,
        number_ty,
    );

    assert!(fixture.is_subtype_type_id_type_id(a, b).is_subtype());
    assert!(fixture.is_subtype_type_id_type_id(b, a).is_subtype());
}
