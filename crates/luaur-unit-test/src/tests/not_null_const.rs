//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/NotNull.test.cpp:119:not_null_const`
//! Source: `tests/NotNull.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/NotNull.test.cpp
//! - incoming:
//!   - declares <- source_file tests/NotNull.test.cpp
//! - outgoing:
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - translates_to -> rust_item not_null_const

#[cfg(test)]
#[test]
fn not_null_const() {
    use luaur_analysis::records::not_null::NotNull;

    let mut p = 0;
    let mut q = 0;

    let n = NotNull::new(&mut p as *mut i32);

    unsafe {
        *n.get() = 123;
    }

    let mut m = n;

    assert_eq!(123, *m);

    let n2 = NotNull::new(&mut q as *mut i32);
    m = n2;

    let m2 = n;
    unsafe {
        *m2.get() = 321;
    }

    assert_eq!(321, *n);
    assert_eq!(m.get(), n2.get());
}
