//! Ported from upstream Luau doctest.
//! Node: `cxx:Test:Luau.UnitTest:tests/Subtyping.test.cpp:1268:subtyping_a_b_string_lower_string`
//! Source: `tests/Subtyping.test.cpp`

use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_a_b_string_lower_string() {
    let mut fixture = SubtypeFixture::default();
    let a_ty = fixture.str("a");
    let b_ty = fixture.str("b");
    let string_ty = fixture.builtin_types.stringType;

    let not_b = fixture.negate(b_ty);
    let not_b_and_string = fixture.meet(not_b, string_ty);
    let sub_ty = fixture.join(a_ty, not_b_and_string);
    let table_with_lower = fixture.table_with_lower();

    assert!(fixture
        .is_subtype_type_id_type_id(sub_ty, table_with_lower)
        .is_subtype());
}
