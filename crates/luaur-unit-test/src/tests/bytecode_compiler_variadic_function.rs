//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCompiler.test.cpp:628:bytecode_compiler_variadic_function`
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
//!   - type_ref -> record BcInst (Bytecode/include/Luau/BytecodeGraph.h)
//!   - calls -> function getOp (tests/BytecodeCompiler.test.cpp)
//!   - type_ref -> enum BcOpKind (Bytecode/include/Luau/BytecodeGraph.h)
//!   - type_ref -> record BcImm (Bytecode/include/Luau/BytecodeGraph.h)
//!   - calls -> method BcFunction::immOp (Bytecode/include/Luau/BytecodeGraph.h)
//!   - type_ref -> enum BcImmKind (Bytecode/include/Luau/BytecodeGraph.h)
//!   - type_ref -> record BcOp (Bytecode/include/Luau/BytecodeGraph.h)
//!   - translates_to -> rust_item bytecode_compiler_variadic_function

#[cfg(test)]
#[test]
fn bytecode_compiler_variadic_function() {
    use crate::functions::check_ops::check_ops;
    use crate::functions::get_op::get_op;
    use crate::records::bytecode_compiler_fixture::BytecodeCompilerFixture;
    use luaur_bytecode::enums::bc_imm_kind::BcImmKind;
    use luaur_bytecode::enums::bc_op_kind::BcOpKind;
    use luaur_common::enums::luau_opcode::LuauOpcode;

    let mut fixture = BytecodeCompilerFixture::new();
    let mut fn_ = fixture
        .build_bytecode(
            r#"
        local function fn(a, ...)
            local b, c = ...
            local l = {...}
            return a + b + c + l[1], ...
        end
    "#,
            0,
        )
        .expect("expected bytecode");

    assert_eq!(fn_.blocks.len(), 2);
    let entry_op = fn_.entry_block;
    let entry = fn_.block_op(entry_op).clone();
    assert!(check_ops(
        &mut fn_,
        &entry.ops,
        &[
            LuauOpcode::LOP_PREPVARARGS,
            LuauOpcode::LOP_GETVARARGS,
            LuauOpcode::LOP_NEWTABLE,
            LuauOpcode::LOP_GETVARARGS,
            LuauOpcode::LOP_SETLIST,
            LuauOpcode::LOP_ADD,
            LuauOpcode::LOP_ADD,
            LuauOpcode::LOP_LOADK,
            LuauOpcode::LOP_GETTABLE,
            LuauOpcode::LOP_ADD,
            LuauOpcode::LOP_GETVARARGS,
            LuauOpcode::LOP_RETURN,
        ]
    ));

    let get_var_args1 = fn_.inst_op(get_op(&entry, 1)).clone();
    assert_eq!(get_var_args1.ops.len(), 2);
    assert_eq!(get_var_args1.ops[0].kind, BcOpKind::VmReg);
    assert_eq!(get_var_args1.ops[0].index, 1);
    assert_eq!(get_var_args1.ops[1].kind, BcOpKind::Imm);
    let get_var_args1_count = *fn_.imm_op(get_var_args1.ops[1]);
    assert_eq!(get_var_args1_count.kind, BcImmKind::Int);
    assert_eq!(unsafe { get_var_args1_count.value.valueInt }, 2);

    let get_var_args2_op = get_op(&entry, 3);
    let get_var_args2 = fn_.inst_op(get_var_args2_op).clone();
    assert_eq!(get_var_args2.ops.len(), 2);
    assert_eq!(get_var_args2.ops[0].kind, BcOpKind::VmReg);
    assert_eq!(get_var_args2.ops[0].index, 4);
    assert_eq!(get_var_args2.ops[1].kind, BcOpKind::Imm);
    let get_var_args2_count = *fn_.imm_op(get_var_args2.ops[1]);
    assert_eq!(get_var_args2_count.kind, BcImmKind::Int);
    assert_eq!(unsafe { get_var_args2_count.value.valueInt }, -1);

    let set_list = fn_.inst_op(get_op(&entry, 4)).clone();
    assert_eq!(set_list.ops.len(), 4);
    let set_list_start_idx = *fn_.imm_op(set_list.ops[0]);
    assert_eq!(set_list_start_idx.kind, BcImmKind::Int);
    assert_eq!(unsafe { set_list_start_idx.value.valueInt }, 1);
    let set_list_count = *fn_.imm_op(set_list.ops[1]);
    assert_eq!(set_list_count.kind, BcImmKind::Int);
    assert_eq!(unsafe { set_list_count.value.valueInt }, -1);
    let new_table_op = get_op(&entry, 2);
    assert_eq!(set_list.ops[2], new_table_op);
    assert_eq!(set_list.ops[3], get_var_args2_op);
}
