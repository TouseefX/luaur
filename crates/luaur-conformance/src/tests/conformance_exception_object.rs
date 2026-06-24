//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:2942:conformance_exception_object`
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
//!   - type_ref -> record ExceptionResult (tests/Conformance.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function lua_newthread (VM/src/lapi.cpp)
//!   - calls -> function lua_getfield (VM/src/lapi.cpp)
//!   - calls -> function lua_isLfunction (VM/src/lapi.cpp)
//!   - calls -> function lua_call (VM/src/lapi.cpp)
//!   - type_ref -> type_alias StateRef (tests/Conformance.test.cpp)
//!   - calls -> function runConformance (tests/Conformance.test.cpp)
//!   - calls -> function lua_newstate (VM/src/lstate.cpp)
//!   - calls -> function limitedRealloc (tests/Conformance.test.cpp)
//!   - calls -> function endsWith (tests/Conformance.test.cpp)
//!   - translates_to -> rust_item conformance_exception_object

#[cfg(test)]
#[test]
fn conformance_exception_object() {
    use crate::functions::conformance_exception_object_capture_exception::conformance_exception_object_capture_exception;
    use crate::functions::ends_with::ends_with;
    use crate::functions::limited_realloc::limited_realloc;
    use crate::functions::run_conformance::runConformance;
    use luaur_vm::functions::lua_newstate::lua_newstate;

    let global_state = runConformance(
        c"exceptions.luau".as_ptr(),
        None,
        None,
        unsafe { lua_newstate(Some(limited_realloc), core::ptr::null_mut()) },
        core::ptr::null_mut(),
        false,
        core::ptr::null_mut(),
    );
    let l = global_state.as_ptr();

    unsafe {
        let result =
            conformance_exception_object_capture_exception(l, c"infinite_recursion_error".as_ptr());
        assert!(result.exception_generated);

        let result = conformance_exception_object_capture_exception(l, c"empty_function".as_ptr());
        assert!(!result.exception_generated);

        let result =
            conformance_exception_object_capture_exception(l, c"pass_number_to_error".as_ptr());
        assert!(result.exception_generated);
        assert!(ends_with(&result.description, "42"));

        let result =
            conformance_exception_object_capture_exception(l, c"pass_string_to_error".as_ptr());
        assert!(result.exception_generated);
        assert!(ends_with(&result.description, "string argument"));

        let result =
            conformance_exception_object_capture_exception(l, c"pass_table_to_error".as_ptr());
        assert!(result.exception_generated);

        let result =
            conformance_exception_object_capture_exception(l, c"large_allocation_error".as_ptr());
        assert!(result.exception_generated);
    }
}
