//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Set.test.cpp:67:set_iterate_over_set`
//! Source: `tests/Set.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Set.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file Analysis/include/Luau/Set.h
//! - incoming:
//!   - declares <- source_file tests/Set.test.cpp
//! - outgoing:
//!   - translates_to -> rust_item set_iterate_over_set

#[cfg(test)]
#[test]
fn set_iterate_over_set() {
    use luaur_analysis::records::set::Set;

    let mut s1 = Set::<i32>::new(0);
    s1.insert(&1);
    s1.insert(&2);
    s1.insert(&3);
    assert_eq!(s1.size(), 3);

    let sum: i32 = s1.iter().copied().sum();
    assert_eq!(sum, 6);
}
