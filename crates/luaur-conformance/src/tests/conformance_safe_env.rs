//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:3907:conformance_safe_env`
//! Source: `tests/Conformance.test.cpp`

#[cfg(test)]
#[test]
fn conformance_safe_env() {
    use crate::functions::run_conformance::runConformance;

    runConformance(
        c"safeenv.luau".as_ptr(),
        None,
        None,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        false,
        core::ptr::null_mut(),
    );
}
