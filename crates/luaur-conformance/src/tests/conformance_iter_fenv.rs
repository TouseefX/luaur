//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:3656:conformance_iter_fenv`
//! Source: `tests/Conformance.test.cpp`

#[cfg(test)]
#[test]
fn conformance_iter_fenv() {
    use crate::functions::run_conformance::runConformance;

    runConformance(
        c"iter_fenv.luau".as_ptr(),
        None,
        None,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        false,
        core::ptr::null_mut(),
    );
}
