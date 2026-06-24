//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:4081:conformance_userdata_direct_access`
//! Source: `tests/Conformance.test.cpp`

#[cfg(test)]
#[test]
fn conformance_userdata_direct_access() {
    use crate::functions::conformance_userdata_direct_access_setup::conformance_userdata_direct_access_setup;
    use crate::functions::get_or_create_atom::reset_direct_atom_state;
    use crate::functions::run_conformance::runConformance;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;

    let _luau_udata_direct_access = ScopedFastFlag::new(&FFlag::LuauUdataDirectAccess6, true);

    reset_direct_atom_state();

    runConformance(
        c"udata_direct.luau".as_ptr(),
        Some(conformance_userdata_direct_access_setup),
        None,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        false,
        core::ptr::null_mut(),
    );
}
