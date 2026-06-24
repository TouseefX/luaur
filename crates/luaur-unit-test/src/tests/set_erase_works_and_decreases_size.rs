//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Set.test.cpp:46:set_erase_works_and_decreases_size`
//! Source: `tests/Set.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Set.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file Analysis/include/Luau/Set.h
//! - incoming:
//!   - declares <- source_file tests/Set.test.cpp
//! - outgoing:
//!   - translates_to -> rust_item set_erase_works_and_decreases_size

#[cfg(test)]
#[test]
fn set_erase_works_and_decreases_size() {
    use luaur_analysis::records::set::Set;

    let mut s1 = Set::<i32>::new(0);
    s1.insert(&1);
    s1.insert(&2);
    assert_eq!(s1.size(), 2);
    assert!(s1.contains(&1));
    assert!(s1.contains(&2));

    s1.erase(&1);
    assert_eq!(s1.size(), 1);
    assert!(!s1.contains(&1));
    assert!(s1.contains(&2));

    s1.erase(&2);
    assert_eq!(s1.size(), 0);
    assert!(s1.empty());
    assert!(!s1.contains(&1));
    assert!(!s1.contains(&2));
}
