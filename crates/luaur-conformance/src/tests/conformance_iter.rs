//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:3640:conformance_iter`
//! Source: `tests/Conformance.test.cpp`

#[cfg(test)]
#[test]
fn conformance_iter() {
    use crate::functions::conformance_iter_setup::conformance_iter_setup;
    use crate::functions::run_conformance::runConformance;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;

    let _luau_yield_iter = ScopedFastFlag::new(&FFlag::LuauYieldIter2, true);

    runConformance(
        c"iter.luau".as_ptr(),
        Some(conformance_iter_setup),
        None,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        false,
        core::ptr::null_mut(),
    );
}
