//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCompiler.test.cpp:709:bytecode_compiler_tables_strings_and_fastcall`
//! Source: `tests/BytecodeCompiler.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/BytecodeCompiler.test.cpp
//! - source_includes:
//!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
//!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
//!   - includes -> source_file Common/include/Luau/BytecodeWire.h
//!   - includes -> source_file Compiler/include/Luau/Compiler.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/BytecodeCompiler.test.cpp
//! - outgoing:
//!   - calls -> method BytecodeCompilerFixture::buildBytecode (tests/BytecodeCompiler.test.cpp)
//!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record BcBlock (Bytecode/include/Luau/BytecodeGraph.h)
//!   - calls -> function checkOps (tests/BytecodeCompiler.test.cpp)
//!   - translates_to -> rust_item bytecode_compiler_tables_strings_and_fastcall

#[cfg(test)]
#[test]
fn bytecode_compiler_tables_strings_and_fastcall() {
    use crate::functions::check_ops::check_ops;
    use crate::records::bytecode_compiler_fixture::BytecodeCompilerFixture;
    use luaur_common::enums::luau_opcode::LuauOpcode;

    let mut fixture = BytecodeCompilerFixture::new();
    let mut fn_ = fixture
        .build_bytecode(
            r#"
        local tt = {}
        local function fn(x)
            local t = { a = x, b = x .. 42 }
            return table.insert({t}, tt)
        end
    "#,
            1,
        )
        .expect("expected bytecode");

    assert_eq!(fn_.blocks.len(), 2);
    let entry_op = fn_.entry_block;
    let entry = fn_.block_op(entry_op).clone();
    assert!(check_ops(
        &mut fn_,
        &entry.ops,
        &[
            LuauOpcode::LOP_DUPTABLE,
            LuauOpcode::LOP_SETTABLEKS,
            LuauOpcode::LOP_MOVE,
            LuauOpcode::LOP_LOADN,
            LuauOpcode::LOP_CONCAT,
            LuauOpcode::LOP_SETTABLEKS,
            LuauOpcode::LOP_NEWTABLE,
            LuauOpcode::LOP_MOVE,
            LuauOpcode::LOP_SETLIST,
            LuauOpcode::LOP_GETUPVAL,
            LuauOpcode::LOP_FASTCALL2,
            LuauOpcode::LOP_GETIMPORT,
            LuauOpcode::LOP_CALL,
            LuauOpcode::LOP_RETURN,
        ]
    ));
}
