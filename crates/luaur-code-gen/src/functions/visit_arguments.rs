use crate::functions::is_pseudo::is_pseudo;
use crate::records::ir_inst::IrInst;

pub fn visit_arguments<F>(inst: &mut IrInst, mut func: F)
where
    F: FnMut(crate::records::ir_op::IrOp),
{
    if is_pseudo(inst.cmd) {
        return;
    }

    for op in inst.ops.iter() {
        func(op.clone());
    }
}
