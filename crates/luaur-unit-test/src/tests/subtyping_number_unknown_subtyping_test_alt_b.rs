//! Ported from upstream Luau doctest.
//! Node: `cxx:Test:Luau.UnitTest:tests/Subtyping.test.cpp:521:subtyping_number_unknown`
//! Source: `tests/Subtyping.test.cpp`

use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_number_unknown() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let unknown_ty = fixture.builtin_types.unknownType;

    assert!(fixture
        .is_subtype_type_id_type_id(number_ty, unknown_ty)
        .is_subtype());
}
