use crate::functions::add_use::add_use;
use crate::functions::remove_use::remove_use;
use crate::records::ir_function::IrFunction;
use crate::records::ir_op::IrOp;

pub fn replace_ir_function_ir_op_ir_op(
    function: &mut IrFunction,
    original: &mut IrOp,
    replacement: IrOp,
) {
    // Add use before removing new one if that's the last one keeping target operand alive
    add_use(function, replacement);
    remove_use(function, *original);

    *original = replacement;
}
