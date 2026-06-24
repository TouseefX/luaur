//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCompiler.test.cpp:565:bytecode_compiler_multi_call_variadic`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method BytecodeCompilerFixture::buildBytecode (tests/BytecodeCompiler.test.cpp)
//!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record BcBlock (Bytecode/include/Luau/BytecodeGraph.h)
//!   - calls -> function checkEdges (tests/BytecodeCompiler.test.cpp)
//!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
//!   - type_ref -> enum BcBlockEdgeKind (Bytecode/include/Luau/BytecodeGraph.h)
//!   - calls -> function fallthroughBlock (tests/BytecodeCompiler.test.cpp)
//!   - calls -> function branchBlock (tests/BytecodeCompiler.test.cpp)
//!   - calls -> function checkOps (tests/BytecodeCompiler.test.cpp)
//!   - type_ref -> record BcInst (Bytecode/include/Luau/BytecodeGraph.h)
//!   - calls -> function getOp (tests/BytecodeCompiler.test.cpp)
//!   - type_ref -> enum BcOpKind (Bytecode/include/Luau/BytecodeGraph.h)
//!   - type_ref -> record BcImm (Bytecode/include/Luau/BytecodeGraph.h)
//!   - calls -> method BcFunction::immOp (Bytecode/include/Luau/BytecodeGraph.h)
//!   - type_ref -> enum BcImmKind (Bytecode/include/Luau/BytecodeGraph.h)
//!   - type_ref -> record BcOp (Bytecode/include/Luau/BytecodeGraph.h)
//!   - translates_to -> rust_item bytecode_compiler_multi_call_variadic

#[cfg(test)]
#[test]
fn bytecode_compiler_multi_call_variadic() {
    use crate::functions::branch_op::branch_op;
    use crate::functions::check_edges::check_edges;
    use crate::functions::check_ops::check_ops;
    use crate::functions::fallthrough_op::fallthrough_op;
    use crate::functions::get_op::get_op;
    use crate::records::bytecode_compiler_fixture::BytecodeCompilerFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_bytecode::enums::bc_block_edge_kind::BcBlockEdgeKind;
    use luaur_bytecode::enums::bc_imm_kind::BcImmKind;
    use luaur_bytecode::enums::bc_op_kind::BcOpKind;
    use luaur_common::enums::luau_opcode::LuauOpcode;

    let _emit_call_feedback = ScopedFastFlag::new(&luaur_common::FFlag::LuauEmitCallFeedback, true);
    let mut fixture = BytecodeCompilerFixture::new();
    let mut fn_ = fixture
        .build_bytecode(
            r#"
        local function fn(n)
            if n > 0 then
                return 0, 1
            else
                local a, b = fn(n - 1)
                return a + b, fn(n)
            end
        end
    "#,
            0,
        )
        .expect("expected bytecode");

    assert_eq!(fn_.blocks.len(), 4);
    let entry_op = fn_.entry_block;
    let entry = fn_.block_op(entry_op).clone();
    assert!(check_edges(
        &entry.successors,
        &[BcBlockEdgeKind::Branch, BcBlockEdgeKind::Fallthrough]
    ));
    let if_true = fn_.block_op(fallthrough_op(&entry.successors)).clone();
    assert!(check_edges(
        &if_true.successors,
        &[BcBlockEdgeKind::Fallthrough]
    ));
    let if_false = fn_.block_op(branch_op(&entry.successors)).clone();
    assert!(check_edges(
        &if_true.successors,
        &[BcBlockEdgeKind::Fallthrough]
    ));

    assert!(check_ops(
        &mut fn_,
        &entry.ops,
        &[LuauOpcode::LOP_LOADK, LuauOpcode::LOP_JUMPIFNOTLT]
    ));
    assert!(check_ops(
        &mut fn_,
        &if_true.ops,
        &[
            LuauOpcode::LOP_LOADK,
            LuauOpcode::LOP_LOADK,
            LuauOpcode::LOP_RETURN,
        ]
    ));
    assert!(check_ops(
        &mut fn_,
        &if_false.ops,
        &[
            LuauOpcode::LOP_GETUPVAL,
            LuauOpcode::LOP_LOADK,
            LuauOpcode::LOP_SUB,
            LuauOpcode::LOP_CALLFB,
            LuauOpcode::LOP_ADD,
            LuauOpcode::LOP_GETUPVAL,
            LuauOpcode::LOP_MOVE,
            LuauOpcode::LOP_CALL,
            LuauOpcode::LOP_RETURN,
        ]
    ));

    let ret = fn_.inst_op(get_op(&if_false, 8)).clone();
    assert_eq!(ret.ops.len(), 3);
    assert_eq!(ret.ops[0].kind, BcOpKind::Imm);
    let ret_count = *fn_.imm_op(ret.ops[0]);
    assert_eq!(ret_count.kind, BcImmKind::Int);
    assert_eq!(unsafe { ret_count.value.valueInt }, -1);
    let add_op = get_op(&if_false, 4);
    assert_eq!(ret.ops[1], add_op);
    let multi_call_op = get_op(&if_false, 7);
    assert_eq!(ret.ops[2], multi_call_op);
}
