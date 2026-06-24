use crate::enums::ir_block_kind::IrBlockKind;
use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::macros::op_a::op_a;
use crate::macros::op_b::op_b;
use crate::records::ir_block::IrBlock;
use crate::records::ir_function::IrFunction;
use crate::records::ir_inst::IrInst;

use crate::functions::replace_inst_operand_ir_utils::replace_ir_function_ir_inst_operand;

pub fn optimize_memory_operands_x_64_ir_function_ir_block(
    function: &mut IrFunction,
    block: &mut IrBlock,
) {
    CODEGEN_ASSERT!(block.kind != IrBlockKind::Dead);

    let start = block.start;
    let finish = block.finish;

    for index in start..=finish {
        CODEGEN_ASSERT!(index < function.instructions.len() as u32);
        let inst: &mut IrInst = &mut function.instructions[index as usize];

        match inst.cmd {
            IrCmd::CHECK_TAG => {
                let inst_op_a = op_a(inst);
                if inst_op_a.kind() == IrOpKind::Inst {
                    let tag = function.inst_op(inst_op_a);

                    if tag.use_count == 1 && tag.cmd == IrCmd::LOAD_TAG && {
                        let tag_op_a = op_a(tag);
                        tag_op_a.kind() == IrOpKind::VmReg || tag_op_a.kind() == IrOpKind::VmConst
                    } {
                        let replacement_op = op_a(tag);
                        replace_ir_function_ir_inst_operand(function, index, 0, replacement_op);
                    }
                }
            }
            IrCmd::CHECK_TRUTHY => {
                // Read both operands from inst before any function calls to satisfy borrow checker
                let inst_op_a = op_a(inst);
                let inst_op_b = op_b(inst.clone());

                if inst_op_a.kind() == IrOpKind::Inst {
                    let tag = function.inst_op(inst_op_a);

                    if tag.use_count == 1 && tag.cmd == IrCmd::LOAD_TAG && {
                        let tag_op_a = op_a(tag);
                        tag_op_a.kind() == IrOpKind::VmReg || tag_op_a.kind() == IrOpKind::VmConst
                    } {
                        let replacement_op = op_a(tag);
                        replace_ir_function_ir_inst_operand(function, index, 0, replacement_op);
                    }
                }

                if inst_op_b.kind() == IrOpKind::Inst {
                    let value = function.inst_op(inst_op_b);

                    if value.use_count == 1 && value.cmd == IrCmd::LOAD_INT {
                        let replacement_op = op_a(value);
                        replace_ir_function_ir_inst_operand(function, index, 1, replacement_op);
                    }
                }
            }
            IrCmd::ADD_NUM
            | IrCmd::SUB_NUM
            | IrCmd::MUL_NUM
            | IrCmd::DIV_NUM
            | IrCmd::IDIV_NUM
            | IrCmd::MOD_NUM
            | IrCmd::MIN_NUM
            | IrCmd::MAX_NUM => {
                let inst_op_b = op_b(inst.clone());
                if inst_op_b.kind() == IrOpKind::Inst {
                    let rhs = function.inst_op(inst_op_b);

                    if rhs.use_count == 1 && rhs.cmd == IrCmd::LOAD_DOUBLE && {
                        let rhs_op_a = op_a(rhs);
                        rhs_op_a.kind() == IrOpKind::VmReg || rhs_op_a.kind() == IrOpKind::VmConst
                    } {
                        let replacement_op = op_a(rhs);
                        replace_ir_function_ir_inst_operand(function, index, 1, replacement_op);
                    }
                }
            }
            IrCmd::JUMP_EQ_TAG => {
                // Read both operands from inst before any function calls to satisfy borrow checker
                let inst_op_a = op_a(inst);
                let inst_op_b = op_b(inst.clone());

                if inst_op_a.kind() == IrOpKind::Inst {
                    let tag_a = function.inst_op(inst_op_a);

                    if tag_a.use_count == 1 && tag_a.cmd == IrCmd::LOAD_TAG && {
                        let tag_a_op_a = op_a(tag_a);
                        tag_a_op_a.kind() == IrOpKind::VmReg
                            || tag_a_op_a.kind() == IrOpKind::VmConst
                    } {
                        let replacement_op = op_a(tag_a);
                        replace_ir_function_ir_inst_operand(function, index, 0, replacement_op);
                        continue;
                    }
                }

                if inst_op_b.kind() == IrOpKind::Inst {
                    let tag_b = function.inst_op(inst_op_b);

                    if tag_b.use_count == 1 && tag_b.cmd == IrCmd::LOAD_TAG && {
                        let tag_b_op_a = op_a(tag_b);
                        tag_b_op_a.kind() == IrOpKind::VmReg
                            || tag_b_op_a.kind() == IrOpKind::VmConst
                    } {
                        let replacement_op = op_a(tag_b);
                        function.instructions[index as usize]
                            .ops
                            .as_mut_slice()
                            .swap(0, 1);

                        replace_ir_function_ir_inst_operand(function, index, 0, replacement_op);
                    }
                }
            }
            IrCmd::JUMP_CMP_NUM => {
                let inst_op_a = op_a(inst);
                if inst_op_a.kind() == IrOpKind::Inst {
                    let num = function.inst_op(inst_op_a);

                    if num.use_count == 1 && num.cmd == IrCmd::LOAD_DOUBLE {
                        let replacement_op = op_a(num);
                        replace_ir_function_ir_inst_operand(function, index, 0, replacement_op);
                    }
                }
            }
            IrCmd::FLOOR_NUM
            | IrCmd::CEIL_NUM
            | IrCmd::ROUND_NUM
            | IrCmd::SQRT_NUM
            | IrCmd::ABS_NUM => {
                let inst_op_a = op_a(inst);
                if inst_op_a.kind() == IrOpKind::Inst {
                    let arg = function.inst_op(inst_op_a);

                    if arg.use_count == 1 && arg.cmd == IrCmd::LOAD_DOUBLE && {
                        let arg_op_a = op_a(arg);
                        arg_op_a.kind() == IrOpKind::VmReg || arg_op_a.kind() == IrOpKind::VmConst
                    } {
                        let replacement_op = op_a(arg);
                        replace_ir_function_ir_inst_operand(function, index, 0, replacement_op);
                    }
                }
            }
            _ => {}
        }
    }
}
