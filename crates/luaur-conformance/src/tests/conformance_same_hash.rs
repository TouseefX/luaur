//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:2237:conformance_same_hash`
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
//!   - calls -> function luaS_hash (VM/src/lstring.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - type_ref -> record BytecodeBuilder (Bytecode/include/Luau/BytecodeBuilder.h)
//!   - calls -> method BytecodeBuilder::getStringHash (Bytecode/src/BytecodeBuilder.cpp)
//!   - translates_to -> rust_item conformance_same_hash

#[cfg(test)]
#[test]
fn conformance_same_hash() {
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_bytecode::records::string_ref::StringRef;
    use luaur_vm::functions::lua_s_hash::luaS_hash;

    fn string_ref(s: &'static [u8]) -> StringRef {
        StringRef::new(s.as_ptr() as *const i8, s.len())
    }

    assert_eq!(
        luaS_hash(c"".as_ptr(), 0),
        BytecodeBuilder::get_string_hash(string_ref(b""))
    );
    assert_eq!(
        luaS_hash(c"lua".as_ptr(), 3),
        BytecodeBuilder::get_string_hash(string_ref(b"lua"))
    );
    assert_eq!(
        luaS_hash(c"luau".as_ptr(), 4),
        BytecodeBuilder::get_string_hash(string_ref(b"luau"))
    );
    assert_eq!(
        luaS_hash(c"luaubytecode".as_ptr(), 12),
        BytecodeBuilder::get_string_hash(string_ref(b"luaubytecode"))
    );
    assert_eq!(
        luaS_hash(c"luaubytecodehash".as_ptr(), 16),
        BytecodeBuilder::get_string_hash(string_ref(b"luaubytecodehash"))
    );

    let buf = [0 as core::ffi::c_char; 128];
    unsafe {
        assert_eq!(
            luaS_hash(buf.as_ptr().add(1), 120),
            luaS_hash(buf.as_ptr().add(2), 120)
        );
    }
}
