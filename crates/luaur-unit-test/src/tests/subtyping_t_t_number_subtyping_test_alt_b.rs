//! Ported from upstream Luau doctest.
//! Node: `cxx:Test:Luau.UnitTest:tests/Subtyping.test.cpp:723:subtyping_t_t_number`
//! Source: `tests/Subtyping.test.cpp`

use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;

#[cfg(test)]
#[test]
fn subtyping_t_t_number() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let generic_t = fixture.generic("T");

    let generic_t_to_nothing_ty = fixture.generic_fn(vec![generic_t], vec![generic_t], vec![]);
    let number_to_nothing_ty =
        fixture.fn_item_initializer_list_type_id_initializer_list_type_id(vec![number_ty], vec![]);

    assert!(fixture
        .is_subtype_type_id_type_id(generic_t_to_nothing_ty, number_to_nothing_ty)
        .is_subtype());
}
