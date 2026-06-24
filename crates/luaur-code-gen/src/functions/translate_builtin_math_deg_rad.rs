use crate::enums::builtin_impl_type::BuiltinImplType;
use crate::enums::ir_cmd::IrCmd;
use crate::functions::builtin_check_double::builtin_check_double;
use crate::functions::builtin_load_double::builtin_load_double;
use crate::records::builtin_impl_result::BuiltinImplResult;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;

pub fn translate_builtin_math_deg_rad(
    build: &mut IrBuilder,
    _cmd: IrCmd,
    nparams: i32,
    ra: i32,
    arg: i32,
    _args: IrOp,
    nresults: i32,
    pcpos: i32,
) -> BuiltinImplResult {
    if nparams < 1 || nresults > 1 {
        return BuiltinImplResult {
            r#type: BuiltinImplType::None,
            actual_result_count: -1,
        };
    }

    let arg_reg = build.vm_reg(arg as u8);
    builtin_check_double(build, arg_reg, pcpos);

    let rpd = 3.14159265358979323846 / 180.0_f64;

    let varg = builtin_load_double(build, arg_reg);
    let rpd_op = build.const_double(rpd);
    let value = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_NUM, varg, rpd_op);
    let ra_reg = build.vm_reg(ra as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, ra_reg, value);

    if ra != arg {
        let tag = build.const_tag(3); // LUA_TNUMBER
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_reg, tag);
    }

    BuiltinImplResult {
        r#type: BuiltinImplType::Full,
        actual_result_count: 1,
    }
}
