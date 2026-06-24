//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:1043:normalize_non_final_types_can_be_normalized_but_are_not_cached`
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
//!   - calls -> method NormalizeFixture::getGlobalScope (tests/Normalize.test.cpp)
//!   - type_ref -> record NormalizedType (Analysis/include/Luau/Normalize.h)
//!   - calls -> method NormalizeFixture::normalize (tests/Normalize.test.cpp)
//!   - translates_to -> rust_item normalize_non_final_types_can_be_normalized_but_are_not_cached

#[cfg(test)]
#[test]
fn normalize_non_final_types_can_be_normalized_but_are_not_cached() {
    use crate::records::normalize_fixture::NormalizeFixture;
    use alloc::sync::Arc;

    let mut fixture = NormalizeFixture::default();
    let scope = fixture.get_global_scope();
    let builtins = fixture.base.builtin_types;
    let a = unsafe {
        fixture
            .arena
            .fresh_type_not_null_builtin_types_scope(&*builtins, scope)
    };

    let na1 = fixture.normalize(a).expect("expected normalized free type");
    let na2 = fixture.normalize(a).expect("expected normalized free type");

    assert!(!Arc::ptr_eq(&na1, &na2));
}
