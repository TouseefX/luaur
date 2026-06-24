//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCompiler.test.cpp:289:bytecode_compiler_repeat_until_loop`
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
//!   - calls -> function fallthroughBlock (tests/BytecodeCompiler.test.cpp)
//!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
//!   - type_ref -> record Loop (Compiler/src/Compiler.cpp)
//!   - calls -> function branchBlock (tests/BytecodeCompiler.test.cpp)
//!   - calls -> function checkOps (tests/BytecodeCompiler.test.cpp)
//!   - type_ref -> record BcOp (Bytecode/include/Luau/BytecodeGraph.h)
//!   - calls -> function getOp (tests/BytecodeCompiler.test.cpp)
//!   - type_ref -> record BcInst (Bytecode/include/Luau/BytecodeGraph.h)
//!   - type_ref -> enum BcOpKind (Bytecode/include/Luau/BytecodeGraph.h)
//!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
//!   - type_ref -> record BcPhi (Bytecode/include/Luau/BytecodeGraph.h)
//!   - calls -> method BcFunction::phiOp (Bytecode/include/Luau/BytecodeGraph.h)
//!   - translates_to -> rust_item bytecode_compiler_repeat_until_loop

#[cfg(test)]
#[test]
fn bytecode_compiler_repeat_until_loop() {
    use crate::functions::branch_op::branch_op;
    use crate::functions::check_edges::check_edges;
    use crate::functions::check_ops::check_ops;
    use crate::functions::fallthrough_op::fallthrough_op;
    use crate::functions::get_op::get_op;
    use crate::records::bytecode_compiler_fixture::BytecodeCompilerFixture;
    use luaur_bytecode::enums::bc_block_edge_kind::BcBlockEdgeKind;
    use luaur_bytecode::enums::bc_op_kind::BcOpKind;
    use luaur_common::enums::luau_opcode::LuauOpcode;

    let mut fixture = BytecodeCompilerFixture::new();
    let mut fn_ = fixture
        .build_bytecode(
            r#"
        function fn()
            local var = 0
            repeat var += 1 until var < 10
        end
    "#,
            0,
        )
        .expect("expected bytecode");

    assert_eq!(fn_.blocks.len(), 5);
    let entry_op = fn_.entry_block;
    let entry = fn_.block_op(entry_op).clone();
    assert!(check_edges(
        &entry.successors,
        &[BcBlockEdgeKind::Fallthrough]
    ));

    let loop_body_op = fallthrough_op(&entry.successors);
    let loop_body = fn_.block_op(loop_body_op).clone();
    assert!(check_edges(
        &loop_body.predecessors,
        &[BcBlockEdgeKind::Fallthrough, BcBlockEdgeKind::Loop]
    ));
    assert!(check_edges(
        &loop_body.successors,
        &[BcBlockEdgeKind::Branch, BcBlockEdgeKind::Fallthrough]
    ));

    let loop_jump_back = fn_.block_op(fallthrough_op(&loop_body.successors)).clone();
    assert!(check_edges(
        &loop_jump_back.successors,
        &[BcBlockEdgeKind::Loop]
    ));
    let ret = fn_.block_op(branch_op(&loop_body.successors)).clone();

    assert!(check_ops(&mut fn_, &entry.ops, &[LuauOpcode::LOP_LOADK]));
    assert!(check_ops(
        &mut fn_,
        &loop_body.ops,
        &[
            LuauOpcode::LOP_LOADK,
            LuauOpcode::LOP_ADD,
            LuauOpcode::LOP_LOADK,
            LuauOpcode::LOP_JUMPIFLT,
        ]
    ));
    assert!(check_ops(
        &mut fn_,
        &loop_jump_back.ops,
        &[LuauOpcode::LOP_JUMPBACK]
    ));
    assert!(check_ops(&mut fn_, &ret.ops, &[LuauOpcode::LOP_RETURN]));

    let var_init_op = get_op(&entry, 0);
    let load_k_one_op = get_op(&loop_body, 0);
    let add_var_op = get_op(&loop_body, 1);
    let add_var = fn_.inst_op(add_var_op).clone();
    assert_eq!(add_var.ops.len(), 2);
    assert_eq!(add_var.ops[0].kind, BcOpKind::Phi);
    let add_var_phi = fn_.phi_op(add_var.ops[0]).clone();
    assert_eq!(add_var_phi.ops[0], var_init_op);
    assert_eq!(add_var_phi.ops[1], add_var_op);
    assert_eq!(add_var.ops[1], load_k_one_op);
}
