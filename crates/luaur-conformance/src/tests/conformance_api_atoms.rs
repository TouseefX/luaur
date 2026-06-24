//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:2772:conformance_api_atoms`
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
//!   - calls -> function luaL_newstate (VM/src/linit.cpp)
//!   - calls -> function lua_close (VM/src/lstate.cpp)
//!   - calls -> function lua_callbacks (VM/src/lapi.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function lua_pushstring (VM/src/lapi.cpp)
//!   - calls -> function lua_concat (VM/src/lapi.cpp)
//!   - calls -> function lua_tostringatom (VM/src/lapi.cpp)
//!   - translates_to -> rust_item conformance_api_atoms

#[cfg(test)]
#[test]
fn conformance_api_atoms() {
    use core::ffi::c_int;
    use std::ffi::CStr;

    use crate::functions::conformance_api_atoms_useratom::conformance_api_atoms_useratom;
    use crate::type_aliases::state_ref::StateRef;
    use luaur_vm::functions::lua_callbacks::lua_callbacks;
    use luaur_vm::functions::lua_concat::lua_concat;
    use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
    use luaur_vm::functions::lua_pushstring::lua_pushstring;
    use luaur_vm::functions::lua_tostringatom::lua_tostringatom;

    let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let l = global_state.as_ptr();

    unsafe {
        (*lua_callbacks(l)).useratom = Some(conformance_api_atoms_useratom);

        lua_pushstring(l, c"string".as_ptr());
        lua_pushstring(l, c"import".as_ptr());
        lua_pushstring(l, c"ant".as_ptr());
        lua_concat(l, 2);
        lua_pushstring(l, c"unimportant".as_ptr());

        let mut a1: c_int = 0;
        let mut a2: c_int = 0;
        let mut a3: c_int = 0;

        let s1 = lua_tostringatom(l, -3, &mut a1);
        let s2 = lua_tostringatom(l, -2, &mut a2);
        let s3 = lua_tostringatom(l, -1, &mut a3);

        assert_eq!(CStr::from_ptr(s1), c"string");
        assert_eq!(a1, 0);

        assert_eq!(CStr::from_ptr(s2), c"important");
        assert_eq!(a2, 1);

        assert_eq!(CStr::from_ptr(s3), c"unimportant");
        assert_eq!(a3, -1);
    }
}
