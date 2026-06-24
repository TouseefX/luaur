//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Set.test.cpp:83:set_iterate_over_set_skips_erased_elements`
//! Source: `tests/Set.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Set.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file Analysis/include/Luau/Set.h
//! - incoming:
//!   - declares <- source_file tests/Set.test.cpp
//! - outgoing:
//!   - translates_to -> rust_item set_iterate_over_set_skips_erased_elements

#[cfg(test)]
#[test]
fn set_iterate_over_set_skips_erased_elements() {
    use luaur_analysis::records::set::Set;

    let mut s1 = Set::<i32>::new(0);
    for value in 1..=6 {
        s1.insert(&value);
    }
    assert_eq!(s1.size(), 6);

    s1.erase(&2);
    s1.erase(&4);
    s1.erase(&6);

    let sum: i32 = s1.iter().copied().sum();
    assert_eq!(sum, 9);
}
