//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:3140:conformance_gc_dump`
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
//!   - calls -> function luaC_dump (VM/src/lgcdebug.cpp)
//!   - calls -> function luaC_enumheap (VM/src/lgcdebug.cpp)
//!   - calls -> method RefinementKeyArena::node (Analysis/src/DataFlowGraph.cpp)
//!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - type_ref -> type_alias StateRef (tests/Conformance.test.cpp)
//!   - calls -> function luaL_newstate (VM/src/linit.cpp)
//!   - calls -> function lua_close (VM/src/lstate.cpp)
//!   - calls -> function lua_createtable (VM/src/lapi.cpp)
//!   - calls -> function lua_pushstring (VM/src/lapi.cpp)
//!   - calls -> function lua_setfield (VM/src/lapi.cpp)
//!   - calls -> function lua_pushinteger (VM/src/lapi.cpp)
//!   - calls -> function lua_rawseti (VM/src/lapi.cpp)
//!   - calls -> function lua_pushvalue (VM/src/lapi.cpp)
//!   - calls -> function lua_setmetatable (VM/src/lapi.cpp)
//!   - calls -> macro lua_newuserdata (VM/include/lua.h)
//!   - calls -> macro lua_pushcclosure (VM/include/lua.h)
//!   - calls -> function lua_silence (tests/Conformance.test.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function lua_newbuffer (VM/src/lapi.cpp)
//!   - calls -> function lua_newthread (VM/src/lapi.cpp)
//!   - calls -> function lua_loadstring (tests/Conformance.test.cpp)
//!   - calls -> function lua_resume (VM/src/ldo.cpp)
//!   - calls -> function luaC_fullgc (VM/src/lgc.cpp)
//!   - type_ref -> record Node (tests/Conformance.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record EnumContext (tests/Conformance.test.cpp)
//!   - type_ref -> record DenseHashMap (Common/include/Luau/DenseHash.h)
//!   - translates_to -> rust_item conformance_gc_dump

#[cfg(test)]
#[test]
fn conformance_gc_dump() {
    use crate::functions::conformance_gc_dump_edge::conformance_gc_dump_edge;
    use crate::functions::conformance_gc_dump_node::conformance_gc_dump_node;
    use crate::functions::lua_loadstring::lua_loadstring;
    use crate::functions::lua_silence::lua_silence;
    use crate::records::conformance_gc_dump_enum_context::ConformanceGcDumpEnumContext;
    use crate::type_aliases::state_ref::StateRef;
    use core::ffi::{c_char, c_int, c_void};
    use luaur_vm::enums::lua_status::lua_Status;
    use luaur_vm::functions::lua_c_dump::luaC_dump;
    use luaur_vm::functions::lua_c_enumheap::luaC_enumheap;
    use luaur_vm::functions::lua_c_fullgc::luaC_fullgc;
    use luaur_vm::functions::lua_createtable::lua_createtable;
    use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
    use luaur_vm::functions::lua_newbuffer::lua_newbuffer;
    use luaur_vm::functions::lua_newthread::lua_newthread;
    use luaur_vm::functions::lua_pushinteger::lua_pushinteger;
    use luaur_vm::functions::lua_pushstring::lua_pushstring;
    use luaur_vm::functions::lua_pushvalue::lua_pushvalue;
    use luaur_vm::functions::lua_rawseti::lua_rawseti;
    use luaur_vm::functions::lua_resume::lua_resume;
    use luaur_vm::functions::lua_setfield::lua_setfield;
    use luaur_vm::functions::lua_setmetatable::lua_setmetatable;
    use luaur_vm::macros::lua_newuserdata::lua_newuserdata;
    use luaur_vm::macros::lua_pushcclosure::lua_pushcclosure;
    use luaur_vm::macros::lua_tostring::lua_tostring;
    use luaur_vm::records::lua_state::lua_State;
    use luaur_vm::type_aliases::lua_c_function::lua_CFunction;
    use std::ffi::{CStr, CString};

    extern "C" {
        fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
        fn fclose(file: *mut c_void) -> c_int;
    }

    let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let l = global_state.as_ptr();

    unsafe {
        lua_createtable(l, 1, 2);
        lua_pushstring(l, c"value".as_ptr());
        lua_setfield(l, -2, c"key".as_ptr());

        lua_pushstring(l, c"u42".as_ptr());
        lua_setfield(l, -2, c"__type".as_ptr());

        lua_pushinteger(l, 42);
        lua_rawseti(l, -2, 1000);

        lua_pushinteger(l, 42);
        lua_rawseti(l, -2, 1);

        lua_pushvalue(l, -1);
        lua_setmetatable(l, -2);

        lua_newuserdata(l, 42);
        lua_pushvalue(l, -2);
        lua_setmetatable(l, -2);

        lua_pushinteger(l, 1);
        let silence: lua_CFunction = Some(core::mem::transmute(
            lua_silence as fn(*mut c_void) -> c_int,
        ));
        lua_pushcclosure(l, silence, c"test".as_ptr(), 1);

        lua_newbuffer(l, 100);

        let cl: *mut lua_State = lua_newthread(l);
        let source = CString::new(
            r#"
local x
x = {}
local function f()
    x[1] = math.abs(42)
end
function foo()
    x[2] = ''
    for i = 1, 10000 do x[2] ..= '1234567890' end
end
foo()
return f
"#,
        )
        .expect("script source contains nul");

        lua_pushstring(cl, source.as_ptr());
        lua_pushstring(cl, c"=GCDump".as_ptr());
        assert_eq!(lua_loadstring(cl), 1);

        let status = lua_resume(cl, core::ptr::null_mut(), 0);
        if status != lua_Status::LUA_OK as c_int {
            let error = CStr::from_ptr(lua_tostring!(cl, -1)).to_string_lossy();
            panic!("GCDump setup chunk failed: {error}");
        }

        #[cfg(windows)]
        let path = c"NUL".as_ptr();
        #[cfg(not(windows))]
        let path = c"/dev/null".as_ptr();

        let file = fopen(path, c"w".as_ptr());
        assert!(!file.is_null());

        luaC_fullgc(l);
        luaC_dump(l, file, None);
        assert_eq!(fclose(file), 0);

        let mut context = ConformanceGcDumpEnumContext::default();
        luaC_enumheap(
            l,
            &mut context as *mut ConformanceGcDumpEnumContext as *mut c_void,
            Some(conformance_gc_dump_node),
            Some(conformance_gc_dump_edge),
        );

        assert!(
            context.errors.is_empty(),
            "GCDump enum validation errors: {:?}",
            context.errors
        );
        assert!(!context.nodes.is_empty());
        assert!(!context.edges.is_empty());
        assert!(context.seen_target_string);
    }
}
