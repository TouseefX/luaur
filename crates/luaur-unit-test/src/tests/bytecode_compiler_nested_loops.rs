//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCompiler.test.cpp:427:bytecode_compiler_nested_loops`
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
//!   - calls -> function checkEdges (tests/BytecodeCompiler.test.cpp)
//!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
//!   - type_ref -> enum BcBlockEdgeKind (Bytecode/include/Luau/BytecodeGraph.h)
//!   - type_ref -> record BcOp (Bytecode/include/Luau/BytecodeGraph.h)
//!   - calls -> function fallthroughOp (tests/BytecodeCompiler.test.cpp)
//!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
//!   - type_ref -> record Loop (Compiler/src/Compiler.cpp)
//!   - calls -> function fallthroughBlock (tests/BytecodeCompiler.test.cpp)
//!   - calls -> function loopOp (tests/BytecodeCompiler.test.cpp)
//!   - calls -> function branchBlock (tests/BytecodeCompiler.test.cpp)
//!   - calls -> function checkOps (tests/BytecodeCompiler.test.cpp)
//!   - calls -> function getOp (tests/BytecodeCompiler.test.cpp)
//!   - type_ref -> record BcInst (Bytecode/include/Luau/BytecodeGraph.h)
//!   - calls -> function isPhiOf (tests/BytecodeCompiler.test.cpp)
//!   - translates_to -> rust_item bytecode_compiler_nested_loops

#[cfg(test)]
#[test]
fn bytecode_compiler_nested_loops() {
    use crate::functions::branch_op::branch_op;
    use crate::functions::check_edges::check_edges;
    use crate::functions::check_ops::check_ops;
    use crate::functions::fallthrough_op::fallthrough_op;
    use crate::functions::get_op::get_op;
    use crate::functions::is_phi_of::is_phi_of;
    use crate::functions::loop_op::loop_op;
    use crate::records::bytecode_compiler_fixture::BytecodeCompilerFixture;
    use luaur_bytecode::enums::bc_block_edge_kind::BcBlockEdgeKind;
    use luaur_common::enums::luau_opcode::LuauOpcode;

    let mut fixture = BytecodeCompilerFixture::new();
    let mut fn_ = fixture
        .build_bytecode(
            r#"
        function fn()
            local res = 0
            local var = 0
            repeat
                local i = 0
                repeat
                    res += i * var
                    i += 1
                until i < 5
                var += 1
            until var < 10
        end
    "#,
            0,
        )
        .expect("expected bytecode");

    assert_eq!(fn_.blocks.len(), 8);
    let entry_op = fn_.entry_block;
    let entry = fn_.block_op(entry_op).clone();
    assert!(check_edges(
        &entry.successors,
        &[BcBlockEdgeKind::Fallthrough]
    ));

    let outer_entry_op = fallthrough_op(&entry.successors);
    let outer_entry = fn_.block_op(outer_entry_op).clone();
    assert!(check_edges(
        &outer_entry.predecessors,
        &[BcBlockEdgeKind::Fallthrough, BcBlockEdgeKind::Loop]
    ));
    assert!(check_edges(
        &outer_entry.successors,
        &[BcBlockEdgeKind::Fallthrough]
    ));

    let inner_entry_op = fallthrough_op(&outer_entry.successors);
    let inner_entry = fn_.block_op(inner_entry_op).clone();
    assert!(check_edges(
        &inner_entry.predecessors,
        &[BcBlockEdgeKind::Fallthrough, BcBlockEdgeKind::Loop]
    ));
    assert!(check_edges(
        &inner_entry.successors,
        &[BcBlockEdgeKind::Branch, BcBlockEdgeKind::Fallthrough]
    ));

    let inner_back_loop = fn_
        .block_op(fallthrough_op(&inner_entry.successors))
        .clone();
    assert!(check_edges(
        &inner_back_loop.successors,
        &[BcBlockEdgeKind::Loop]
    ));
    assert_eq!(loop_op(&inner_back_loop.successors), inner_entry_op);

    let outer_epllog = fn_.block_op(branch_op(&inner_entry.successors)).clone();
    assert!(check_edges(
        &outer_epllog.successors,
        &[BcBlockEdgeKind::Branch, BcBlockEdgeKind::Fallthrough]
    ));
    let outer_back_loop = fn_
        .block_op(fallthrough_op(&outer_epllog.successors))
        .clone();
    assert!(check_edges(
        &outer_back_loop.successors,
        &[BcBlockEdgeKind::Loop]
    ));
    assert_eq!(loop_op(&outer_back_loop.successors), outer_entry_op);
    let ret = fn_.block_op(branch_op(&outer_epllog.successors)).clone();

    assert!(check_ops(
        &mut fn_,
        &entry.ops,
        &[LuauOpcode::LOP_LOADK, LuauOpcode::LOP_LOADK]
    ));
    assert!(check_ops(
        &mut fn_,
        &outer_entry.ops,
        &[LuauOpcode::LOP_LOADK]
    ));
    assert!(check_ops(
        &mut fn_,
        &inner_entry.ops,
        &[
            LuauOpcode::LOP_MUL,
            LuauOpcode::LOP_ADD,
            LuauOpcode::LOP_LOADK,
            LuauOpcode::LOP_ADD,
            LuauOpcode::LOP_LOADK,
            LuauOpcode::LOP_JUMPIFLT,
        ]
    ));
    assert!(check_ops(
        &mut fn_,
        &inner_back_loop.ops,
        &[LuauOpcode::LOP_JUMPBACK]
    ));
    assert!(check_ops(
        &mut fn_,
        &outer_epllog.ops,
        &[
            LuauOpcode::LOP_LOADK,
            LuauOpcode::LOP_ADD,
            LuauOpcode::LOP_LOADK,
            LuauOpcode::LOP_JUMPIFLT,
        ]
    ));
    assert!(check_ops(
        &mut fn_,
        &outer_back_loop.ops,
        &[LuauOpcode::LOP_JUMPBACK]
    ));
    assert!(check_ops(&mut fn_, &ret.ops, &[LuauOpcode::LOP_RETURN]));

    let var_init_op = get_op(&entry, 1);
    let var_inc_op = get_op(&outer_epllog, 1);
    let i_init_op = get_op(&outer_entry, 0);
    let i_inc_op = get_op(&inner_entry, 3);
    let i_times_var_op = get_op(&inner_entry, 0);
    let i_times_var = fn_.inst_op(i_times_var_op).clone();
    assert_eq!(i_times_var.ops.len(), 2);
    assert!(is_phi_of(&mut fn_, i_times_var.ops[0], i_init_op, i_inc_op));
    assert!(is_phi_of(
        &mut fn_,
        i_times_var.ops[1],
        var_init_op,
        var_inc_op
    ));
}
