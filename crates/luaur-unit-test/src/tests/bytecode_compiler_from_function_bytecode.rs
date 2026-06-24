//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCompiler.test.cpp:220:bytecode_compiler_from_function_bytecode`
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
//!   - calls -> method SubtypeFixture::meta (tests/Subtyping.test.cpp)
//!   - type_ref -> record BcBlock (Bytecode/include/Luau/BytecodeGraph.h)
//!   - type_ref -> record Entry (Ast/include/Luau/Lexer.h)
//!   - calls -> function checkEdges (tests/BytecodeCompiler.test.cpp)
//!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
//!   - type_ref -> enum BcBlockEdgeKind (Bytecode/include/Luau/BytecodeGraph.h)
//!   - type_ref -> record BcOp (Bytecode/include/Luau/BytecodeGraph.h)
//!   - calls -> function branchOp (tests/BytecodeCompiler.test.cpp)
//!   - calls -> function fallthroughOp (tests/BytecodeCompiler.test.cpp)
//!   - type_ref -> record BcInst (Bytecode/include/Luau/BytecodeGraph.h)
//!   - calls -> method BcInstHelper::op (Bytecode/include/Luau/BytecodeOps.h)
//!   - type_ref -> enum BcOpKind (Bytecode/include/Luau/BytecodeGraph.h)
//!   - type_ref -> enum BcVmConstKind (Bytecode/include/Luau/BytecodeGraph.h)
//!   - calls -> function checkOps (tests/BytecodeCompiler.test.cpp)
//!   - translates_to -> rust_item bytecode_compiler_from_function_bytecode

#[cfg(test)]
#[test]
fn bytecode_compiler_from_function_bytecode() {
    use crate::functions::branch_op::branch_op;
    use crate::functions::check_edges::check_edges;
    use crate::functions::check_ops::check_ops;
    use crate::functions::fallthrough_op::fallthrough_op;
    use crate::records::bytecode_compiler_fixture::BytecodeCompilerFixture;
    use luaur_bytecode::enums::bc_block_edge_kind::BcBlockEdgeKind;
    use luaur_bytecode::enums::bc_op_kind::BcOpKind;
    use luaur_bytecode::enums::bc_vm_const_kind::BcVmConstKind;
    use luaur_common::enums::luau_opcode::LuauOpcode;

    let mut fixture = BytecodeCompilerFixture::new();
    let mut fn_ = fixture
        .build_bytecode(
            r#"
        function fn(a, b)
            local extra = 0
            if a > b then extra = 1 end
            return extra + a + b
        end
    "#,
            0,
        )
        .expect("expected bytecode");

    assert_eq!(fn_.nups, 0);
    assert_eq!(fn_.numparams, 2);
    assert_eq!(fn_.constants.len(), 2);

    assert_eq!(fn_.blocks.len(), 4);
    let entry_op = fn_.entry_block;
    let entry = fn_.block_op(entry_op).clone();
    assert!(check_edges(
        &entry.successors,
        &[BcBlockEdgeKind::Branch, BcBlockEdgeKind::Fallthrough]
    ));

    let cond_false_op = branch_op(&entry.successors);
    let cond_true = fn_.block_op(entry.successors[1].target).clone();
    assert!(check_edges(
        &cond_true.successors,
        &[BcBlockEdgeKind::Fallthrough]
    ));
    assert_eq!(fallthrough_op(&cond_true.successors), cond_false_op);

    let cond_false = fn_.block_op(cond_false_op).clone();
    assert!(check_edges(
        &cond_false.successors,
        &[BcBlockEdgeKind::Fallthrough]
    ));
    assert_eq!(fallthrough_op(&cond_false.successors), fn_.exit_block);
    let exit_op = fn_.exit_block;
    let exit = fn_.block_op(exit_op).clone();

    assert_eq!(entry.ops.len(), 2);
    let mut ops = entry.ops.iter();
    let load_k_op = *ops.next().expect("entry loadk");
    let load_k = fn_.inst_op(load_k_op).clone();
    assert_eq!(load_k.op, LuauOpcode::LOP_LOADK);
    assert_eq!(load_k.ops.len(), 1);
    assert_eq!(load_k.ops[0].kind, BcOpKind::VmConst);
    assert_eq!(load_k.ops[0].index, 0);
    assert_eq!(fn_.constants[0].kind, BcVmConstKind::Number);
    assert_eq!(unsafe { fn_.constants[0].value.valueNumber }, 0.0);

    let jump_if_not_lt = fn_.inst_op(*ops.next().expect("entry jump")).clone();
    assert_eq!(jump_if_not_lt.op, LuauOpcode::LOP_JUMPIFNOTLT);
    assert_eq!(jump_if_not_lt.ops.len(), 3);

    assert!(check_ops(
        &mut fn_,
        &cond_true.ops,
        &[LuauOpcode::LOP_LOADK]
    ));
    assert!(check_ops(
        &mut fn_,
        &cond_false.ops,
        &[
            LuauOpcode::LOP_ADD,
            LuauOpcode::LOP_ADD,
            LuauOpcode::LOP_RETURN,
        ]
    ));
    assert!(check_ops(&mut fn_, &exit.ops, &[]));
}
