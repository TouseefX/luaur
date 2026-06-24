//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Set.test.cpp:34:set_clear_resets_size`
//! Source: `tests/Set.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Set.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file Analysis/include/Luau/Set.h
//! - incoming:
//!   - declares <- source_file tests/Set.test.cpp
//! - outgoing:
//!   - translates_to -> rust_item set_clear_resets_size

#[cfg(test)]
#[test]
fn set_clear_resets_size() {
    use luaur_analysis::records::set::Set;

    let mut s1 = Set::<i32>::new(0);
    s1.insert(&1);
    s1.insert(&2);
    assert_eq!(s1.size(), 2);

    s1.clear();
    assert_eq!(s1.size(), 0);
    assert!(s1.empty());
}
