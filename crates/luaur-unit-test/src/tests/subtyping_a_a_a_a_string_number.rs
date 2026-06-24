use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;

#[cfg(test)]
#[test]
fn subtyping_a_a_a_a_string_number() {
    let mut fixture = SubtypeFixture::default();
    let generic_as = fixture.generic_pack("A");
    let empty_type_pack = fixture.builtin_types.emptyTypePack;
    let string_ty = fixture.builtin_types.stringType;
    let number_ty = fixture.builtin_types.numberType;

    let as_to_nothing = fixture.generic_pack_fn(vec![generic_as], generic_as, empty_type_pack);
    let as_to_nothing_pack = fixture.pack_initializer_list_type_id(vec![as_to_nothing]);
    let f1 = fixture.generic_pack_fn(vec![generic_as], generic_as, as_to_nothing_pack);

    let number_to_nothing =
        fixture.fn_item_initializer_list_type_id_initializer_list_type_id(vec![number_ty], vec![]);
    let f2 = fixture.fn_item_initializer_list_type_id_initializer_list_type_id(
        vec![string_ty],
        vec![number_to_nothing],
    );

    assert!(fixture.is_subtype_type_id_type_id(f1, f2).is_subtype());
}
