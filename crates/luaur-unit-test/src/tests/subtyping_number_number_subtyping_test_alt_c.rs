//! Ported from upstream Luau doctest.
//! Node: `cxx:Test:Luau.UnitTest:tests/Subtyping.test.cpp:1279:subtyping_number_number`
//! Source: `tests/Subtyping.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Subtyping.test.cpp
//! - source_includes:
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Analysis/include/Luau/Instantiation2.h
//!   - includes -> source_file Analysis/include/Luau/TypeFwd.h
//!   - includes -> source_file Analysis/include/Luau/TypePath.h
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/Subtyping.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypePack.h
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/RegisterCallbacks.h
//! - incoming:
//!   - declares <- source_file tests/Subtyping.test.cpp
//! - outgoing:
//!   - calls -> method SubtypeFixture::negate (tests/Subtyping.test.cpp)
//!   - translates_to -> rust_item subtyping_number_number

use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_number_number() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let not_number = fixture.negate(number_ty);
    let not_not_number = fixture.negate(not_number);

    assert!(fixture
        .is_subtype_type_id_type_id(number_ty, not_not_number)
        .is_subtype());
}
