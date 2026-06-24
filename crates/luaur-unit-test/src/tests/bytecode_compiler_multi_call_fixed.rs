//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCompiler.test.cpp:514:bytecode_compiler_multi_call_fixed`
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
//!   - type_ref -> record BcBlock (Bytecode/include/Luau/BytecodeGraph.h)
//!   - calls -> function checkOps (tests/BytecodeCompiler.test.cpp)
//!   - type_ref -> record BcOp (Bytecode/include/Luau/BytecodeGraph.h)
//!   - calls -> function getOp (tests/BytecodeCompiler.test.cpp)
//!   - type_ref -> record BcInst (Bytecode/include/Luau/BytecodeGraph.h)
//!   - type_ref -> enum BcOpKind (Bytecode/include/Luau/BytecodeGraph.h)
//!   - type_ref -> record BcProj (Bytecode/include/Luau/BytecodeGraph.h)
//!   - calls -> method BcFunction::projOp (Bytecode/include/Luau/BytecodeGraph.h)
//!   - calls -> method BcInstHelper::op (Bytecode/include/Luau/BytecodeOps.h)
//!   - type_ref -> record BcImm (Bytecode/include/Luau/BytecodeGraph.h)
//!   - calls -> method BcFunction::immOp (Bytecode/include/Luau/BytecodeGraph.h)
//!   - type_ref -> enum BcImmKind (Bytecode/include/Luau/BytecodeGraph.h)
//!   - translates_to -> rust_item bytecode_compiler_multi_call_fixed

#[cfg(test)]
#[test]
fn bytecode_compiler_multi_call_fixed() {
    use crate::functions::check_ops::check_ops;
    use crate::functions::get_op::get_op;
    use crate::records::bytecode_compiler_fixture::BytecodeCompilerFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_bytecode::enums::bc_imm_kind::BcImmKind;
    use luaur_bytecode::enums::bc_op_kind::BcOpKind;
    use luaur_common::enums::luau_opcode::LuauOpcode;

    let _emit_call_feedback = ScopedFastFlag::new(&luaur_common::FFlag::LuauEmitCallFeedback, true);
    let mut fixture = BytecodeCompilerFixture::new();
    let mut fn_ = fixture
        .build_bytecode(
            r#"
        local function x()
            local a, b = f()
            return b, a
        end
    "#,
            0,
        )
        .expect("expected bytecode");

    let entry_op = fn_.entry_block;
    let entry = fn_.block_op(entry_op).clone();
    assert!(check_ops(
        &mut fn_,
        &entry.ops,
        &[
            LuauOpcode::LOP_GETGLOBAL,
            LuauOpcode::LOP_CALLFB,
            LuauOpcode::LOP_MOVE,
            LuauOpcode::LOP_MOVE,
            LuauOpcode::LOP_RETURN,
        ]
    ));

    let call_op = get_op(&entry, 1);
    let move1_op = get_op(&entry, 2);
    let move1 = fn_.inst_op(move1_op).clone();
    assert_eq!(move1.ops.len(), 1);
    assert_eq!(move1.ops[0].kind, BcOpKind::Proj);
    let move1_proj = fn_.proj_op(move1.ops[0]).clone();
    assert_eq!(move1_proj.op, call_op);
    assert_eq!(move1_proj.index, 1);

    let move2_op = get_op(&entry, 3);
    let move2 = fn_.inst_op(move2_op).clone();
    assert_eq!(move2.ops.len(), 1);
    assert_eq!(move2.ops[0].kind, BcOpKind::Proj);
    let move2_proj = fn_.proj_op(move2.ops[0]).clone();
    assert_eq!(move2_proj.op, call_op);
    assert_eq!(move2_proj.index, 0);

    let ret = fn_.inst_op(get_op(&entry, 4)).clone();
    assert_eq!(ret.ops.len(), 3);
    assert_eq!(ret.ops[0].kind, BcOpKind::Imm);
    let ret_count = *fn_.imm_op(ret.ops[0]);
    assert_eq!(ret_count.kind, BcImmKind::Int);
    assert_eq!(unsafe { ret_count.value.valueInt }, 2);
    assert_eq!(ret.ops[1], move1_op);
    assert_eq!(ret.ops[2], move2_op);
}
