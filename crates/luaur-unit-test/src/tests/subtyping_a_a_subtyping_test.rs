use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;

#[cfg(test)]
#[test]
fn subtyping_a_a() {
    let mut fixture = SubtypeFixture::default();
    let generic_as = fixture.generic_pack("A");

    let generic_nothing_to_as_ty = fixture.generic_pack_fn(
        vec![generic_as],
        fixture.builtin_types.emptyTypePack,
        generic_as,
    );
    let nothing_to_nothing_ty =
        fixture.fn_item_initializer_list_type_id_initializer_list_type_id(vec![], vec![]);

    assert!(fixture
        .is_subtype_type_id_type_id(generic_nothing_to_as_ty, nothing_to_nothing_ty)
        .is_subtype());
}
