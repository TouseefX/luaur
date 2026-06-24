use luaur_bytecode::records::bc_op::BcOp;
use luaur_bytecode::type_aliases::comp_time_bc_function::CompTimeBcFunction;
use luaur_common::enums::luau_opcode::LuauOpcode;

pub fn check_ops(
    fn_: &mut CompTimeBcFunction,
    ops: &alloc::collections::VecDeque<BcOp>,
    expected_ops: &[LuauOpcode],
) -> bool {
    if ops.len() != expected_ops.len() {
        return false;
    }

    for (op, expected) in ops.iter().zip(expected_ops.iter()) {
        if fn_.inst_op(*op).op != *expected {
            return false;
        }
    }

    true
}
