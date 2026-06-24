//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/NotNull.test.cpp:47:not_null_basic_stuff`
//! Source: `tests/NotNull.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/NotNull.test.cpp
//! - incoming:
//!   - declares <- source_file tests/NotNull.test.cpp
//! - outgoing:
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> method WeirdIter::good (Analysis/src/Unifier.cpp)
//!   - type_ref -> record Test (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item not_null_basic_stuff

#[cfg(test)]
#[test]
fn not_null_basic_stuff() {
    use crate::records::test::Test;
    use luaur_analysis::records::not_null::NotNull;

    fn bar(_q: *mut i32) {}

    let mut a_box = Box::new(55);
    let mut b_box = Box::new(55);

    let mut a = NotNull::new(&mut *a_box as *mut i32);
    let b = NotNull::new(&mut *b_box as *mut i32);

    let mut d = a;

    let e = *d;
    *d = 1;
    assert_eq!(e, 55);

    let f = d;
    unsafe {
        *f.get() = 5;
    }

    assert_eq!(a, d);
    assert_ne!(a, b);

    let g = a;
    assert_eq!(g, a);

    let mut t_box = Box::new(Test::new());
    let t = NotNull::new(&mut *t_box as *mut Test);
    unsafe {
        (*t.get()).x = 5;
        (*t.get()).y = 3.14;
    }

    let u = t;
    unsafe {
        (*u.get()).x = 44;
    }
    let v = unsafe { (*u.get()).x };
    assert_eq!(v, 44);

    bar(a.get());

    drop(a_box);
    drop(b_box);
    drop(t_box);

    assert_eq!(0, Test::count());
}
