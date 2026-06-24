//! Ported from upstream Luau doctest.
//! Node: `cxx:Test:Luau.UnitTest:tests/Subtyping.test.cpp:601:subtyping_hello_world_number`
//! Source: `tests/Subtyping.test.cpp`

use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_hello_world_number() {
    let mut fixture = SubtypeFixture::default();
    let hello_ty = fixture.str("hello");
    let world_ty = fixture.str("world");
    let hello_and_world_ty = fixture.meet(hello_ty, world_ty);
    let number_ty = fixture.builtin_types.numberType;

    assert!(fixture
        .is_subtype_type_id_type_id(hello_and_world_ty, number_ty)
        .is_subtype());
}
