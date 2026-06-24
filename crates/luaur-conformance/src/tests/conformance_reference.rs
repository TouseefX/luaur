//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:2255:conformance_reference`
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
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - calls -> function lua_newuserdatadtor (VM/src/lapi.cpp)
//!   - calls -> function lua_gc (VM/src/lapi.cpp)
//!   - calls -> function lua_ref (VM/src/lapi.cpp)
//!   - calls -> macro lua_pop (VM/include/lua.h)
//!   - calls -> macro lua_getref (VM/include/lua.h)
//!   - calls -> function lua_isuserdata (VM/src/lapi.cpp)
//!   - calls -> function lua_unref (VM/src/lapi.cpp)
//!   - translates_to -> rust_item conformance_reference

#[cfg(test)]
#[test]
fn conformance_reference() {
    use std::sync::atomic::Ordering;

    use crate::functions::conformance_reference_dtor::conformance_reference_dtor;
    use crate::functions::conformance_reference_dtor_hits::CONFORMANCE_REFERENCE_DTOR_HITS;
    use crate::type_aliases::state_ref::StateRef;
    use luaur_vm::enums::lua_gc_op::lua_GCOp;
    use luaur_vm::functions::lua_gc::lua_gc;
    use luaur_vm::functions::lua_isuserdata::lua_isuserdata;
    use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
    use luaur_vm::functions::lua_newuserdatadtor::lua_newuserdatadtor;
    use luaur_vm::functions::lua_ref::lua_ref;
    use luaur_vm::functions::lua_unref::lua_unref;
    use luaur_vm::macros::lua_getref::lua_getref;
    use luaur_vm::macros::lua_pop::lua_pop;

    CONFORMANCE_REFERENCE_DTOR_HITS.store(0, Ordering::SeqCst);

    let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let l = global_state.as_ptr();

    unsafe {
        lua_newuserdatadtor(l, 0, Some(conformance_reference_dtor));
        lua_newuserdatadtor(l, 0, Some(conformance_reference_dtor));

        lua_gc(l, lua_GCOp::LUA_GCCOLLECT as i32, 0);
        assert_eq!(CONFORMANCE_REFERENCE_DTOR_HITS.load(Ordering::SeqCst), 0);

        let reference = lua_ref(l, -2);
        lua_pop(l, 2);

        lua_gc(l, lua_GCOp::LUA_GCCOLLECT as i32, 0);
        assert_eq!(CONFORMANCE_REFERENCE_DTOR_HITS.load(Ordering::SeqCst), 1);

        lua_getref(l, reference);
        assert_ne!(lua_isuserdata(l, -1), 0);
        lua_pop(l, 1);

        lua_gc(l, lua_GCOp::LUA_GCCOLLECT as i32, 0);
        assert_eq!(CONFORMANCE_REFERENCE_DTOR_HITS.load(Ordering::SeqCst), 1);

        lua_unref(l, reference);

        lua_gc(l, lua_GCOp::LUA_GCCOLLECT as i32, 0);
        assert_eq!(CONFORMANCE_REFERENCE_DTOR_HITS.load(Ordering::SeqCst), 2);
    }
}
