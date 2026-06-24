//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Set.test.cpp:132:set_erase_using_const_ref_argument`
//! Source: `tests/Set.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Set.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file Analysis/include/Luau/Set.h
//! - incoming:
//!   - declares <- source_file tests/Set.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item set_erase_using_const_ref_argument

#[cfg(test)]
#[test]
fn set_erase_using_const_ref_argument() {
    use alloc::string::String;
    use luaur_analysis::records::set::Set;

    let mut s1 = Set::<String>::new(String::new());
    s1.insert(&String::from("x"));
    s1.insert(&String::from("y"));

    let key = String::from("y");
    s1.erase(&key);

    assert!(s1.count(&String::from("x")) != 0);
    assert_eq!(s1.count(&String::from("y")), 0);
}
