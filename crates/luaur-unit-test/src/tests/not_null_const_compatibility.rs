//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/NotNull.test.cpp:146:not_null_const_compatibility`
//! Source: `tests/NotNull.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/NotNull.test.cpp
//! - incoming:
//!   - declares <- source_file tests/NotNull.test.cpp
//! - outgoing:
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - translates_to -> rust_item not_null_const_compatibility

#[cfg(test)]
#[test]
fn not_null_const_compatibility() {
    use luaur_analysis::records::not_null::NotNull;

    let mut raw = Box::new(8);

    let a = NotNull::new(&mut *raw as *mut i32);
    let _b = NotNull::new(&mut *raw as *mut i32);
    let c = a;

    assert_eq!(*c, 8);
}
