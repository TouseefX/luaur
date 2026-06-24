use crate::enums::builtin_impl_type::BuiltinImplType;
use crate::enums::ir_cmd::IrCmd;
use crate::records::builtin_impl_result::BuiltinImplResult;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;

pub fn translate_builtin_assert(
    build: &mut IrBuilder,
    nparams: i32,
    _ra: i32,
    arg: i32,
    _args: IrOp,
    nresults: i32,
    pcpos: i32,
) -> BuiltinImplResult {
    if nparams < 1 || nresults != 0 {
        return BuiltinImplResult {
            r#type: BuiltinImplType::None,
            actual_result_count: -1,
        };
    }

    let arg_reg = build.vm_reg(arg as u8);
    let tag = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, arg_reg);

    let arg_reg = build.vm_reg(arg as u8);
    let value = build.inst_ir_cmd_ir_op(IrCmd::LOAD_INT, arg_reg);

    let exit_op = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TRUTHY, tag, value, exit_op);

    BuiltinImplResult {
        r#type: BuiltinImplType::UsesFallback,
        actual_result_count: 0,
    }
}
