//! Ported from upstream Luau doctest.
//! Node: `cxx:Test:Luau.UnitTest:tests/Subtyping.test.cpp:812:subtyping_a_a_a`
//! Source: `tests/Subtyping.test.cpp`

use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;

#[cfg(test)]
#[test]
fn subtyping_a_a_a() {
    let mut fixture = SubtypeFixture::default();
    let generic_as = fixture.generic_pack("A");

    let nothing_to_nothing_ty =
        fixture.fn_item_initializer_list_type_id_initializer_list_type_id(vec![], vec![]);
    let generic_as_to_as_ty = fixture.generic_pack_fn(vec![generic_as], generic_as, generic_as);

    assert!(!fixture
        .is_subtype_type_id_type_id(nothing_to_nothing_ty, generic_as_to_as_ty)
        .is_subtype());
}
