use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::replace_ir_utils_alt_b::replace_ir_function_ir_block_u32_ir_inst;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::macros::has_op_d::HAS_OP_D;
use crate::records::ir_block::IrBlock;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_function::IrFunction;
use crate::records::ir_inst::IrInst;
use crate::records::ir_op::IrOp;
use crate::records::remove_dead_store_state::RemoveDeadStoreState;
use crate::records::store_reg_info::StoreRegInfo;
use crate::type_aliases::ir_ops::IrOps;

fn make_split(target_op: IrOp, tag_op: IrOp, value_op: IrOp) -> IrInst {
    let mut ops = IrOps::new();
    ops.push_back(target_op);
    ops.push_back(tag_op);
    ops.push_back(value_op);
    IrInst {
        cmd: IrCmd::STORE_SPLIT_TVALUE,
        ops,
        ..IrInst::default()
    }
}

pub fn try_replace_value_with_full_store(
    state: &mut RemoveDeadStoreState,
    build: &mut IrBuilder,
    function: &mut IrFunction,
    block: &mut IrBlock,
    inst_index: u32,
    target_op: IrOp,
    value_op: IrOp,
    reg_info: &mut StoreRegInfo,
) -> bool {
    // If the tag+value pair is established, we can mark both as dead and use a single split TValue store
    if reg_info.tag_inst_idx != !0u32 && reg_info.value_inst_idx != !0u32 {
        let prev_tag_op = function.instructions[reg_info.tag_inst_idx as usize].ops[1];
        let prev_tag = function.tag_op(prev_tag_op);

        CODEGEN_ASSERT!(reg_info.known_tag == prev_tag);
        let repl = make_split(target_op, prev_tag_op, value_op);
        replace_ir_function_ir_block_u32_ir_inst(function, block, inst_index, repl);

        state.kill_tag_store(reg_info);
        state.kill_value_store(reg_info);

        reg_info.tvalue_inst_idx = inst_index;
        return true;
    }

    // We can also replace a dead split TValue store with a new one, while keeping the value the same
    if reg_info.tvalue_inst_idx != !0u32 {
        let prev_cmd = function.instructions[reg_info.tvalue_inst_idx as usize].cmd;

        if prev_cmd == IrCmd::STORE_SPLIT_TVALUE {
            let prev_tag_op = function.instructions[reg_info.tvalue_inst_idx as usize].ops[1];
            let prev_tag = function.tag_op(prev_tag_op);

            CODEGEN_ASSERT!(reg_info.known_tag == prev_tag);
            CODEGEN_ASSERT!(!HAS_OP_D!(
                function.instructions[reg_info.tvalue_inst_idx as usize]
            ));
            let repl = make_split(target_op, prev_tag_op, value_op);
            replace_ir_function_ir_block_u32_ir_inst(function, block, inst_index, repl);

            CODEGEN_ASSERT!(reg_info.tag_inst_idx == !0u32 && reg_info.value_inst_idx == !0u32);
            state.kill_t_value_store(reg_info);

            reg_info.tvalue_inst_idx = inst_index;
            return true;
        } else if prev_cmd == IrCmd::STORE_VECTOR {
            let prev_tag_op = function.instructions[reg_info.tvalue_inst_idx as usize].ops[4];
            CODEGEN_ASSERT!(prev_tag_op.kind() != IrOpKind::None);
            let prev_tag = function.tag_op(prev_tag_op);

            CODEGEN_ASSERT!(reg_info.known_tag == prev_tag);
            let repl = make_split(target_op, prev_tag_op, value_op);
            replace_ir_function_ir_block_u32_ir_inst(function, block, inst_index, repl);

            CODEGEN_ASSERT!(reg_info.tag_inst_idx == !0u32 && reg_info.value_inst_idx == !0u32);
            state.kill_t_value_store(reg_info);

            reg_info.tvalue_inst_idx = inst_index;
            return true;
        } else if prev_cmd == IrCmd::STORE_TVALUE
            && reg_info.known_tag != 0xff
            && reg_info.tag_inst_idx == !0u32
        {
            let prev_tag_op = build.const_tag(reg_info.known_tag);
            let repl = make_split(target_op, prev_tag_op, value_op);
            replace_ir_function_ir_block_u32_ir_inst(function, block, inst_index, repl);

            CODEGEN_ASSERT!(reg_info.tag_inst_idx == !0u32 && reg_info.value_inst_idx == !0u32);
            state.kill_t_value_store(reg_info);

            reg_info.tvalue_inst_idx = inst_index;
            return true;
        }
    }

    false
}
