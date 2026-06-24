//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:383:normalize_error_suppression`
//! Source: `tests/Normalize.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Normalize.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//! - incoming:
//!   - declares <- source_file tests/Normalize.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
//!   - translates_to -> rust_item normalize_error_suppression

#[cfg(test)]
#[test]
fn normalize_error_suppression() {
    use crate::records::is_subtype_fixture::IsSubtypeFixture;
    use alloc::string::String;
    use luaur_common::FFlag;

    let mut fixture = IsSubtypeFixture::default();

    fixture
        .base
        .check_string_optional_frontend_options(&String::from(""), None);

    let (any, err, str_ty, unk) = {
        let builtins = fixture.base.get_builtins();
        (
            builtins.anyType,
            builtins.errorType,
            builtins.stringType,
            builtins.unknownType,
        )
    };

    assert!(!fixture.is_subtype(any, err));
    assert!(fixture.is_subtype(err, any));

    assert!(!fixture.is_subtype(any, str_ty));
    assert!(fixture.is_subtype(str_ty, any));

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert!(fixture.is_subtype(any, unk));
    } else {
        assert!(!fixture.is_subtype(any, unk));
    }

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert!(fixture.is_subtype(err, str_ty));
    } else {
        assert!(!fixture.is_subtype(err, str_ty));
    }

    assert!(!fixture.is_subtype(str_ty, err));

    assert!(!fixture.is_subtype(err, unk));
    assert!(!fixture.is_subtype(unk, err));

    assert!(fixture.is_subtype(str_ty, unk));
    assert!(!fixture.is_subtype(unk, str_ty));
}
