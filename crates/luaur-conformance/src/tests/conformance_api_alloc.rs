//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:2929:conformance_api_alloc`
//! Source: `tests/Conformance.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Conformance.test.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file VM/include/lua.h
//!   - includes -> source_file VM/include/lualib.h
//!   - includes -> source_file Compiler/include/luacode.h
//!   - includes -> source_file CodeGen/include/luacodegen.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Compiler/include/Luau/Compiler.h
//!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
//!   - includes -> source_file CodeGen/include/Luau/BytecodeSummary.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file tests/ConformanceIrHooks.h
//! - incoming:
//!   - declares <- source_file tests/Conformance.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias StateRef (tests/Conformance.test.cpp)
//!   - calls -> function lua_newstate (VM/src/lstate.cpp)
//!   - calls -> function limitedRealloc (tests/Conformance.test.cpp)
//!   - calls -> function lua_close (VM/src/lstate.cpp)
//!   - calls -> function lua_getallocf (VM/src/lapi.cpp)
//!   - translates_to -> rust_item conformance_api_alloc

#[cfg(test)]
#[test]
fn conformance_api_alloc() {
    use core::ffi::c_void;

    use crate::functions::limited_realloc::limited_realloc;
    use crate::type_aliases::state_ref::StateRef;
    use luaur_vm::functions::lua_getallocf::lua_getallocf;
    use luaur_vm::functions::lua_newstate::lua_newstate;

    let mut ud = 0;
    let global_state =
        StateRef::new(unsafe { lua_newstate(Some(limited_realloc), (&mut ud as *mut i32).cast()) })
            .expect("lua state allocation failed");
    let l = global_state.as_ptr();

    let mut ud_check: *mut c_void = core::ptr::null_mut();
    let allocf = lua_getallocf(l, &mut ud_check);
    let expected = limited_realloc
        as unsafe extern "C" fn(*mut c_void, *mut c_void, usize, usize) -> *mut c_void;

    assert!(matches!(
        allocf,
        Some(f) if std::ptr::fn_addr_eq(f, expected)
    ));
    assert_eq!(ud_check, (&mut ud as *mut i32).cast());
}
