use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::macros::op_a::op_a;
use crate::macros::op_b::op_b;
use crate::macros::opt_op_a::OPT_OP_A;
use crate::macros::opt_op_b::OPT_OP_B;
use crate::records::buffer_access_base::BufferAccessBase;
use crate::records::const_prop_state::ConstPropState;
use crate::records::ir_op::IrOp;

impl ConstPropState {
    pub fn get_offset_base(&mut self, value: IrOp) -> BufferAccessBase {
        let mut base = BufferAccessBase {
            op: value,
            scale: 1,
            offset: 0,
        };

        loop {
            if base.op.kind() != IrOpKind::Inst {
                break;
            }

            let inst = unsafe { (*self.function).inst_op(base.op).clone() };
            let lhs_num = unsafe { (*self.function).as_double_op(OPT_OP_A(inst.clone())) };
            let rhs_num = unsafe { (*self.function).as_double_op(OPT_OP_B(inst.clone())) };
            let lhs_int = unsafe { (*self.function).as_int_op(OPT_OP_A(inst.clone())) };
            let rhs_int = unsafe { (*self.function).as_int_op(OPT_OP_B(inst.clone())) };

            if inst.cmd == IrCmd::ADD_NUM
                && lhs_num.is_some()
                && self.is_valid_double_for_immediate(lhs_num.unwrap())
            {
                base.offset += (lhs_num.unwrap() as i32) * base.scale;
                base.op = op_b(inst);
            } else if inst.cmd == IrCmd::ADD_NUM
                && rhs_num.is_some()
                && self.is_valid_double_for_immediate(rhs_num.unwrap())
            {
                base.offset += (rhs_num.unwrap() as i32) * base.scale;
                base.op = op_a(&mut inst.clone());
            } else if inst.cmd == IrCmd::SUB_NUM
                && rhs_num.is_some()
                && self.is_valid_double_for_immediate(rhs_num.unwrap())
            {
                base.offset -= (rhs_num.unwrap() as i32) * base.scale;
                base.op = op_a(&mut inst.clone());
            } else if inst.cmd == IrCmd::MUL_NUM
                && lhs_num.is_some()
                && self.is_valid_double_for_immediate(lhs_num.unwrap())
            {
                base.scale *= lhs_num.unwrap() as i32;
                base.op = op_b(inst);
            } else if inst.cmd == IrCmd::MUL_NUM
                && rhs_num.is_some()
                && self.is_valid_double_for_immediate(rhs_num.unwrap())
            {
                base.scale *= rhs_num.unwrap() as i32;
                base.op = op_a(&mut inst.clone());
            } else if inst.cmd == IrCmd::ADD_INT
                && lhs_int.is_some()
                && self.is_valid_integer_for_immediate(lhs_int.unwrap())
            {
                base.offset += lhs_int.unwrap() * base.scale;
                base.op = op_b(inst);
            } else if inst.cmd == IrCmd::ADD_INT
                && rhs_int.is_some()
                && self.is_valid_integer_for_immediate(rhs_int.unwrap())
            {
                base.offset += rhs_int.unwrap() * base.scale;
                base.op = op_a(&mut inst.clone());
            } else if inst.cmd == IrCmd::SUB_INT
                && rhs_int.is_some()
                && self.is_valid_integer_for_immediate(rhs_int.unwrap())
            {
                base.offset -= rhs_int.unwrap() * base.scale;
                base.op = op_a(&mut inst.clone());
            } else if inst.cmd == IrCmd::TRUNCATE_UINT {
                base.op = op_a(&mut inst.clone());
            } else {
                break;
            }

            if !self.is_valid_integer_for_immediate(base.offset)
                || !self.is_valid_integer_for_immediate(base.scale)
            {
                break;
            }
        }

        base
    }
}
