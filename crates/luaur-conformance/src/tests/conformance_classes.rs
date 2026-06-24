//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:4135:conformance_classes`
//! Source: `tests/Conformance.test.cpp`

#[cfg(test)]
#[test]
fn conformance_classes() {
    use crate::functions::run_conformance::runConformance;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;

    let _debug_luau_user_defined_classes =
        ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _debug_luau_user_defined_classes_runtime =
        ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClassesRuntime, true);

    runConformance(
        c"classes.luau".as_ptr(),
        None,
        None,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        false,
        core::ptr::null_mut(),
    );
}
