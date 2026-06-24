//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:3684:conformance_userdata`
//! Source: `tests/Conformance.test.cpp`

#[cfg(test)]
#[test]
fn conformance_userdata() {
    use crate::functions::conformance_userdata_setup::conformance_userdata_setup;
    use crate::functions::run_conformance::runConformance;

    runConformance(
        c"userdata.luau".as_ptr(),
        Some(conformance_userdata_setup),
        None,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        false,
        core::ptr::null_mut(),
    );
}
