//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Set.test.cpp:106:set_iterate_over_set_skips_first_element_if_it_is_erased`
//! Source: `tests/Set.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Set.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file Analysis/include/Luau/Set.h
//! - incoming:
//!   - declares <- source_file tests/Set.test.cpp
//! - outgoing:
//!   - type_ref -> record DenseHashSet (Common/include/Luau/DenseHash.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item set_iterate_over_set_skips_first_element_if_it_is_erased

#[cfg(test)]
#[test]
fn set_iterate_over_set_skips_first_element_if_it_is_erased() {
    use alloc::string::String;
    use alloc::vec::Vec;
    use luaur_analysis::records::set::Set;

    let mut s1 = Set::<String>::new(String::new());
    s1.insert(&String::from("x"));
    s1.insert(&String::from("y"));
    s1.erase(&String::from("y"));

    let out: Vec<String> = s1.iter().cloned().collect();
    assert_eq!(1, out.len());
}
