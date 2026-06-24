//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/NotNull.test.cpp:99:not_null_hashable`
//! Source: `tests/NotNull.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/NotNull.test.cpp
//! - incoming:
//!   - declares <- source_file tests/NotNull.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method Symbol::c_str (Analysis/include/Luau/Symbol.h)
//!   - translates_to -> rust_item not_null_hashable

#[cfg(test)]
#[test]
fn not_null_hashable() {
    use luaur_analysis::records::not_null::NotNull;
    use std::collections::HashMap;

    let mut a_ = 8;
    let mut b_ = 10;

    let a = NotNull::new(&mut a_ as *mut i32);
    let b = NotNull::new(&mut b_ as *mut i32);

    let hello = "hello";
    let world = "world";

    let mut map = HashMap::new();
    map.insert(a, hello);
    map.insert(b, world);

    assert_eq!(2, map.len());
    assert_eq!(hello, map[&a]);
    assert_eq!(world, map[&b]);
}
