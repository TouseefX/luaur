//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Set.test.cpp:12:set_empty_set_size_0`
//! Source: `tests/Set.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Set.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file Analysis/include/Luau/Set.h
//! - incoming:
//!   - declares <- source_file tests/Set.test.cpp
//! - outgoing:
//!   - translates_to -> rust_item set_empty_set_size_0

#[cfg(test)]
#[test]
fn set_empty_set_size_0() {
    use luaur_analysis::records::set::Set;

    let s1 = Set::<i32>::new(0);
    assert_eq!(s1.size(), 0);
    assert!(s1.empty());
}
