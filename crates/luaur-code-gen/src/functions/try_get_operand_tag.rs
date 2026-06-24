use crate::enums::ir_cmd::IrCmd;
use crate::macros::has_op_c::HAS_OP_C;
use crate::macros::op_c::op_c;
use crate::records::ir_function::IrFunction;
use crate::records::ir_inst::IrInst;
use crate::records::ir_op::IrOp;

pub fn try_get_operand_tag(function: &mut IrFunction, op: IrOp) -> Option<u8> {
    let arg_ptr = function.as_inst_op(op);
    if !arg_ptr.is_null() {
        let arg = unsafe { &*arg_ptr };
        if arg.cmd == IrCmd::TAG_VECTOR {
            return Some(5); // LUA_TVECTOR
        }

        if arg.cmd == IrCmd::LOAD_TVALUE && HAS_OP_C!(arg) {
            let op_c = op_c(arg.clone());
            return Some(function.tag_op(op_c));
        }
    }

    None
}
