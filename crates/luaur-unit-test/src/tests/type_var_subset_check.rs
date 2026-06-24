//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:49:type_var_subset_check`
//! Source: `tests/TypeVar.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeVar.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeVar.test.cpp
//! - outgoing:
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - calls -> function isSubset (Analysis/src/Type.cpp)
//!   - translates_to -> rust_item type_var_subset_check

#[cfg(test)]
#[test]
fn type_var_subset_check() {
    use crate::records::fixture::Fixture;
    use alloc::vec;
    use luaur_analysis::functions::is_subset::is_subset;
    use luaur_analysis::records::union_type::UnionType;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();
    let super_ = UnionType {
        options: vec![
            builtins.numberType,
            builtins.stringType,
            builtins.booleanType,
        ],
    };
    let sub = UnionType {
        options: vec![builtins.numberType, builtins.stringType],
    };
    let not_sub = UnionType {
        options: vec![builtins.numberType, builtins.nilType],
    };

    assert!(is_subset(&super_, &sub));
    assert!(!is_subset(&super_, &not_sub));
}
