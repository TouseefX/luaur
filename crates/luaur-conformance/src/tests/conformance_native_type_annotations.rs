//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:3989:conformance_native_type_annotations`
//! Source: `tests/Conformance.test.cpp`

#[cfg(test)]
#[test]
fn conformance_native_type_annotations() {
    use crate::functions::conformance_native_type_annotations_setup::conformance_native_type_annotations_setup;
    use crate::functions::run_conformance::{runConformance, CODEGEN};
    use luaur_code_gen::functions::luau_codegen_supported::luau_codegen_supported;

    if unsafe { !CODEGEN } || luau_codegen_supported() == 0 {
        return;
    }

    runConformance(
        c"native_types.luau".as_ptr(),
        Some(conformance_native_type_annotations_setup),
        None,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        false,
        core::ptr::null_mut(),
    );
}
