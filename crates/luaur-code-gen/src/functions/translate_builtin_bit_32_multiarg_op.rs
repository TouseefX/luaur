use crate::enums::builtin_impl_type::BuiltinImplType;
use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_condition::IrCondition;
use crate::functions::builtin_check_double::builtin_check_double;
use crate::functions::builtin_load_double::builtin_load_double;
use crate::functions::vm_reg_op::vm_reg_op;
use crate::records::builtin_impl_result::BuiltinImplResult;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;

// File-scope constant from C++ source
const K_BIT32_BINARY_OP_UNROLLED_PARAMS: i32 = 5;

pub fn translate_builtin_bit_32_multiarg_op(
    build: &mut IrBuilder,
    cmd: IrCmd,
    btest: bool,
    nparams: i32,
    ra: i32,
    arg: i32,
    args: IrOp,
    arg3: IrOp,
    nresults: i32,
    pcpos: i32,
) -> BuiltinImplResult {
    if nparams < 1 || nparams > K_BIT32_BINARY_OP_UNROLLED_PARAMS || nresults > 1 {
        return BuiltinImplResult {
            r#type: BuiltinImplType::None,
            actual_result_count: -1,
        };
    }

    let vm_reg_arg = build.vm_reg(arg as u8);
    builtin_check_double(build, vm_reg_arg, pcpos);

    if nparams >= 2 {
        builtin_check_double(build, args, pcpos);
    }

    if nparams >= 3 {
        builtin_check_double(build, arg3, pcpos);
    }

    for i in 4..=nparams {
        let reg_index = vm_reg_op(args) + (i - 2);
        let vm_reg = build.vm_reg(reg_index as u8);
        builtin_check_double(build, vm_reg, pcpos);
    }

    let arg_reg = build.vm_reg(arg as u8);
    let va = builtin_load_double(build, arg_reg);
    let mut res = build.inst_ir_cmd_ir_op(IrCmd::NUM_TO_UINT, va);

    if nparams >= 2 {
        let vb = builtin_load_double(build, args);
        let arg_op = build.inst_ir_cmd_ir_op(IrCmd::NUM_TO_UINT, vb);
        res = build.inst_ir_cmd_ir_op_ir_op(cmd, res, arg_op);
    }

    if nparams >= 3 {
        let vc = builtin_load_double(build, arg3);
        let arg_op = build.inst_ir_cmd_ir_op(IrCmd::NUM_TO_UINT, vc);
        res = build.inst_ir_cmd_ir_op_ir_op(cmd, res, arg_op);
    }

    for i in 4..=nparams {
        let reg = build.vm_reg((vm_reg_op(args) + (i - 2)) as u8);
        let vc = builtin_load_double(build, reg);
        let arg_op = build.inst_ir_cmd_ir_op(IrCmd::NUM_TO_UINT, vc);
        res = build.inst_ir_cmd_ir_op_ir_op(cmd, res, arg_op);
    }

    if btest {
        let zero = build.const_int(0);
        let cond = build.cond(IrCondition::NotEqual);
        let value = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CMP_INT, res, zero, cond);
        let ra_reg = build.vm_reg(ra as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, ra_reg, value);
        let t_boolean = build.const_tag(0x01); // LUA_TBOOLEAN
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_reg, t_boolean);
    } else {
        let value = build.inst_ir_cmd_ir_op(IrCmd::UINT_TO_NUM, res);
        let ra_reg = build.vm_reg(ra as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, ra_reg, value);

        if ra != arg {
            let t_number = build.const_tag(luaur_vm::enums::lua_type::lua_Type::LUA_TNUMBER as u8);
            build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_reg, t_number);
        }
    }

    BuiltinImplResult {
        r#type: BuiltinImplType::Full,
        actual_result_count: 1,
    }
}
