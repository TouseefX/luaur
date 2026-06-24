//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCompiler.test.cpp:343:bytecode_compiler_for_loop_and_backward_input`
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
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record BcBlock (Bytecode/include/Luau/BytecodeGraph.h)
//!   - type_ref -> record Entry (Ast/include/Luau/Lexer.h)
//!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function checkEdges (tests/BytecodeCompiler.test.cpp)
//!   - type_ref -> enum BcBlockEdgeKind (Bytecode/include/Luau/BytecodeGraph.h)
//!   - type_ref -> record BcOp (Bytecode/include/Luau/BytecodeGraph.h)
//!   - calls -> function fallthroughOp (tests/BytecodeCompiler.test.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
//!   - type_ref -> record Loop (Compiler/src/Compiler.cpp)
//!   - calls -> function fallthroughBlock (tests/BytecodeCompiler.test.cpp)
//!   - calls -> function branchOp (tests/BytecodeCompiler.test.cpp)
//!   - calls -> function loopOp (tests/BytecodeCompiler.test.cpp)
//!   - calls -> function checkOps (tests/BytecodeCompiler.test.cpp)
//!   - type_ref -> record BcInst (Bytecode/include/Luau/BytecodeGraph.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
//!   - calls -> function isPhiOf (tests/BytecodeCompiler.test.cpp)
//!   - translates_to -> rust_item bytecode_compiler_for_loop_and_backward_input

#[cfg(test)]
#[test]
fn bytecode_compiler_for_loop_and_backward_input() {
    use crate::functions::branch_op::branch_op;
    use crate::functions::check_edges::check_edges;
    use crate::functions::check_ops::check_ops;
    use crate::functions::fallthrough_op::fallthrough_op;
    use crate::functions::get_op::get_op;
    use crate::functions::is_phi_of::is_phi_of;
    use crate::functions::loop_op::loop_op;
    use crate::records::bytecode_compiler_fixture::BytecodeCompilerFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_bytecode::enums::bc_block_edge_kind::BcBlockEdgeKind;
    use luaur_common::enums::luau_opcode::LuauOpcode;

    let _emit_call_feedback = ScopedFastFlag::new(&luaur_common::FFlag::LuauEmitCallFeedback, true);
    let mut fixture = BytecodeCompilerFixture::new();
    let mut fn_ = fixture
        .build_bytecode(
            r#"
        function fn()
            local var = 3
            for i = 1, 10 do
                if var > 0 then print(i) end
                var -= 1;
            end
        end
    "#,
            0,
        )
        .expect("expected bytecode");

    assert_eq!(fn_.blocks.len(), 6);
    let entry_op = fn_.entry_block;
    let entry = fn_.block_op(entry_op).clone();
    assert_eq!(entry.successors.len(), 2);
    assert!(check_edges(
        &entry.successors,
        &[BcBlockEdgeKind::Branch, BcBlockEdgeKind::Fallthrough]
    ));

    let loop_enter_op = fallthrough_op(&entry.successors);
    let loop_enter = fn_.block_op(loop_enter_op).clone();
    assert!(check_edges(
        &loop_enter.predecessors,
        &[BcBlockEdgeKind::Fallthrough, BcBlockEdgeKind::Loop]
    ));
    assert!(check_edges(
        &loop_enter.successors,
        &[BcBlockEdgeKind::Branch, BcBlockEdgeKind::Fallthrough]
    ));

    let loop_cond = fn_.block_op(fallthrough_op(&loop_enter.successors)).clone();
    assert!(check_edges(
        &loop_cond.successors,
        &[BcBlockEdgeKind::Fallthrough]
    ));
    let loop_epllog_op = branch_op(&loop_enter.successors);
    assert_eq!(fallthrough_op(&loop_cond.successors), loop_epllog_op);
    let loop_epllog = fn_.block_op(loop_epllog_op).clone();
    assert!(check_edges(
        &loop_epllog.successors,
        &[BcBlockEdgeKind::Loop, BcBlockEdgeKind::Fallthrough]
    ));
    assert_eq!(loop_op(&loop_epllog.successors), loop_enter_op);
    let ret = fn_.block_op(loop_epllog.successors[1].target).clone();

    assert!(check_ops(
        &mut fn_,
        &entry.ops,
        &[
            LuauOpcode::LOP_LOADK,
            LuauOpcode::LOP_LOADK,
            LuauOpcode::LOP_LOADK,
            LuauOpcode::LOP_LOADN,
            LuauOpcode::LOP_FORNPREP,
        ]
    ));
    assert!(check_ops(
        &mut fn_,
        &loop_enter.ops,
        &[LuauOpcode::LOP_LOADK, LuauOpcode::LOP_JUMPIFNOTLT]
    ));

    let var_init_op = get_op(&entry, 0);
    let sub_var_op = get_op(&loop_epllog, 1);
    let jump_if_not_lt = fn_.inst_op(get_op(&loop_enter, 1)).clone();
    assert_eq!(jump_if_not_lt.ops.len(), 3);
    assert_eq!(jump_if_not_lt.ops[0], get_op(&loop_enter, 0));
    assert!(is_phi_of(
        &mut fn_,
        jump_if_not_lt.ops[1],
        var_init_op,
        sub_var_op
    ));
    assert_eq!(jump_if_not_lt.ops[2], loop_epllog_op);

    let sub_var = fn_.inst_op(sub_var_op).clone();
    assert_eq!(sub_var.ops.len(), 2);
    assert!(is_phi_of(&mut fn_, sub_var.ops[0], var_init_op, sub_var_op));
    assert_eq!(sub_var.ops[1], get_op(&loop_epllog, 0));

    assert!(check_ops(
        &mut fn_,
        &loop_cond.ops,
        &[
            LuauOpcode::LOP_GETGLOBAL,
            LuauOpcode::LOP_MOVE,
            LuauOpcode::LOP_CALLFB,
        ]
    ));
    assert!(check_ops(
        &mut fn_,
        &loop_epllog.ops,
        &[
            LuauOpcode::LOP_LOADK,
            LuauOpcode::LOP_SUB,
            LuauOpcode::LOP_FORNLOOP,
        ]
    ));
    assert!(check_ops(&mut fn_, &ret.ops, &[LuauOpcode::LOP_RETURN]));
}
