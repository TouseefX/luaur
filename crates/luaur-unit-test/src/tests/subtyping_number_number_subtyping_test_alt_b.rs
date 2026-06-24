//! Ported from upstream Luau doctest.
//! Node: `cxx:Test:Luau.UnitTest:tests/Subtyping.test.cpp:531:subtyping_number_number`
//! Source: `tests/Subtyping.test.cpp`

use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_number_number() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let optional_number_ty = fixture.builtin_types.optionalNumberType;

    assert!(fixture
        .is_subtype_type_id_type_id(number_ty, optional_number_ty)
        .is_subtype());
}
