use crate::enums::ir_cmd::IrCmd;
use crate::functions::is_gco::is_gco;
use crate::functions::replace_ir_utils_alt_b::replace_ir_function_ir_block_u32_ir_inst;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::macros::has_op_d::HAS_OP_D;
use crate::macros::has_op_e::HAS_OP_E;
use crate::records::ir_block::IrBlock;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_function::IrFunction;
use crate::records::ir_inst::IrInst;
use crate::records::ir_op::IrOp;
use crate::records::remove_dead_store_state::RemoveDeadStoreState;
use crate::records::store_reg_info::StoreRegInfo;
use crate::type_aliases::ir_ops::IrOps;
use luaur_vm::enums::lua_type::lua_Type;

fn make_inst(cmd: IrCmd, ops_slice: &[IrOp]) -> IrInst {
    let mut ops = IrOps::new();
    for op in ops_slice {
        ops.push_back(*op);
    }
    IrInst {
        cmd,
        ops,
        ..IrInst::default()
    }
}

pub fn try_replace_tag_with_full_store(
    state: &mut RemoveDeadStoreState,
    _build: &mut IrBuilder,
    function: &mut IrFunction,
    block: &mut IrBlock,
    inst_index: u32,
    target_op: IrOp,
    tag_op: IrOp,
    reg_info: &mut StoreRegInfo,
) -> bool {
    let tag = function.tag_op(tag_op);
    let nil = lua_Type::LUA_TNIL as u8;

    // If the tag+value pair is established, we can mark both as dead and use a single split TValue store
    if reg_info.tag_inst_idx != !0u32
        && (reg_info.value_inst_idx != !0u32 || reg_info.known_tag == nil)
    {
        if tag != nil && reg_info.value_inst_idx != !0u32 {
            let prev_cmd = function.instructions[reg_info.value_inst_idx as usize].cmd;

            if prev_cmd == IrCmd::STORE_VECTOR {
                CODEGEN_ASSERT!(!HAS_OP_E!(
                    function.instructions[reg_info.value_inst_idx as usize]
                ));
                let prev_value_x = function.instructions[reg_info.value_inst_idx as usize].ops[1];
                let prev_value_y = function.instructions[reg_info.value_inst_idx as usize].ops[2];
                let prev_value_z = function.instructions[reg_info.value_inst_idx as usize].ops[3];
                let repl = make_inst(
                    IrCmd::STORE_VECTOR,
                    &[target_op, prev_value_x, prev_value_y, prev_value_z, tag_op],
                );
                replace_ir_function_ir_block_u32_ir_inst(function, block, inst_index, repl);
            } else {
                let prev_value_op = function.instructions[reg_info.value_inst_idx as usize].ops[1];
                let repl = make_inst(
                    IrCmd::STORE_SPLIT_TVALUE,
                    &[target_op, tag_op, prev_value_op],
                );
                replace_ir_function_ir_block_u32_ir_inst(function, block, inst_index, repl);
            }
        }

        state.kill_tag_store(reg_info);
        state.kill_value_store(reg_info);

        reg_info.tvalue_inst_idx = inst_index;
        reg_info.maybe_gco = is_gco(tag);
        reg_info.known_tag = tag;
        state.has_gco_to_clear |= reg_info.maybe_gco;
        return true;
    }

    // We can also replace a dead split TValue store with a new one, while keeping the value the same
    if reg_info.tvalue_inst_idx != !0u32 {
        let prev_cmd = function.instructions[reg_info.tvalue_inst_idx as usize].cmd;

        if prev_cmd == IrCmd::STORE_SPLIT_TVALUE {
            CODEGEN_ASSERT!(!HAS_OP_D!(
                function.instructions[reg_info.tvalue_inst_idx as usize]
            ));

            // If the 'nil' is stored, we keep 'STORE_TAG Rn, tnil' as it writes the 'full' TValue
            if tag != nil {
                let prev_value_op = function.instructions[reg_info.tvalue_inst_idx as usize].ops[2];
                let repl = make_inst(
                    IrCmd::STORE_SPLIT_TVALUE,
                    &[target_op, tag_op, prev_value_op],
                );
                replace_ir_function_ir_block_u32_ir_inst(function, block, inst_index, repl);
            }

            CODEGEN_ASSERT!(reg_info.tag_inst_idx == !0u32 && reg_info.value_inst_idx == !0u32);
            state.kill_t_value_store(reg_info);

            reg_info.tvalue_inst_idx = inst_index;
            reg_info.maybe_gco = is_gco(tag);
            reg_info.known_tag = tag;
            state.has_gco_to_clear |= reg_info.maybe_gco;
            return true;
        } else if prev_cmd == IrCmd::STORE_VECTOR {
            if tag != nil {
                let prev_value_x = function.instructions[reg_info.tvalue_inst_idx as usize].ops[1];
                let prev_value_y = function.instructions[reg_info.tvalue_inst_idx as usize].ops[2];
                let prev_value_z = function.instructions[reg_info.tvalue_inst_idx as usize].ops[3];
                let repl = make_inst(
                    IrCmd::STORE_VECTOR,
                    &[target_op, prev_value_x, prev_value_y, prev_value_z, tag_op],
                );
                replace_ir_function_ir_block_u32_ir_inst(function, block, inst_index, repl);
            }

            CODEGEN_ASSERT!(reg_info.tag_inst_idx == !0u32 && reg_info.value_inst_idx == !0u32);
            state.kill_t_value_store(reg_info);

            reg_info.tvalue_inst_idx = inst_index;
            reg_info.maybe_gco = is_gco(tag);
            reg_info.known_tag = tag;
            state.has_gco_to_clear |= reg_info.maybe_gco;
            return true;
        }
    }

    false
}
