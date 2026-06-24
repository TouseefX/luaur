use crate::enums::builtin_impl_type::BuiltinImplType;
use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_condition::IrCondition;
use crate::functions::builtin_check_double::builtin_check_double;
use crate::functions::builtin_load_double::builtin_load_double;
use crate::functions::vm_reg_op::vm_reg_op;
use crate::records::builtin_impl_result::BuiltinImplResult;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_builtin_bit_32_replace(
    build: &mut IrBuilder,
    nparams: i32,
    ra: i32,
    arg: i32,
    args: IrOp,
    arg3: IrOp,
    nresults: i32,
    pcpos: i32,
) -> BuiltinImplResult {
    if nparams < 3 || nresults > 1 {
        return BuiltinImplResult {
            r#type: BuiltinImplType::None,
            actual_result_count: -1,
        };
    }

    let arg_reg = build.vm_reg(arg as u8);
    builtin_check_double(build, arg_reg, pcpos);
    builtin_check_double(build, args, pcpos);
    builtin_check_double(build, arg3, pcpos);

    let arg_reg = build.vm_reg(arg as u8);
    let va = builtin_load_double(build, arg_reg);
    let vb = builtin_load_double(build, args);
    let vc = builtin_load_double(build, arg3);

    let n = build.inst_ir_cmd_ir_op(IrCmd::NUM_TO_UINT, va);
    let v = build.inst_ir_cmd_ir_op(IrCmd::NUM_TO_UINT, vb);
    let f = build.inst_ir_cmd_ir_op(IrCmd::NUM_TO_INT, vc);

    let value = if nparams == 3 {
        let c32 = build.const_int(32);
        let cond = build.cond(IrCondition::UnsignedLess);
        let exit = build.vm_exit(pcpos as u32);
        build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CHECK_CMP_INT, f, c32, cond, exit);

        let m = build.const_int(1);
        let shift = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BITLSHIFT_UINT, m, f);
        let not_ = build.inst_ir_cmd_ir_op(IrCmd::BITNOT_UINT, shift);
        let lhs = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BITAND_UINT, n, not_);

        let vm = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BITAND_UINT, v, m);
        let rhs = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BITLSHIFT_UINT, vm, f);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::BITOR_UINT, lhs, rhs)
    } else {
        let reg = build.vm_reg((vm_reg_op(args) + 2) as u8);
        builtin_check_double(build, reg, pcpos);
        let reg = build.vm_reg((vm_reg_op(args) + 2) as u8);
        let vd = builtin_load_double(build, reg);

        let w = build.inst_ir_cmd_ir_op(IrCmd::NUM_TO_INT, vd);
        let fw = build.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_INT, f, w);

        let zero = build.const_int(0);
        let ge = build.cond(IrCondition::GreaterEqual);
        let exit = build.vm_exit(pcpos as u32);
        build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CHECK_CMP_INT, f, zero, ge, exit);

        let zero = build.const_int(0);
        let gt = build.cond(IrCondition::Greater);
        let exit = build.vm_exit(pcpos as u32);
        build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CHECK_CMP_INT, w, zero, gt, exit);

        let c32 = build.const_int(32);
        let le = build.cond(IrCondition::LessEqual);
        let exit = build.vm_exit(pcpos as u32);
        build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CHECK_CMP_INT, fw, c32, le, exit);

        let one = build.const_int(1);
        let w_minus_1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SUB_INT, w, one);
        let base = build.const_int(0xfffffffeu32 as i32);
        let shift1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BITLSHIFT_UINT, base, w_minus_1);
        let m = build.inst_ir_cmd_ir_op(IrCmd::BITNOT_UINT, shift1);

        let shift2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BITLSHIFT_UINT, m, f);
        let not_ = build.inst_ir_cmd_ir_op(IrCmd::BITNOT_UINT, shift2);
        let lhs = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BITAND_UINT, n, not_);

        let vm = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BITAND_UINT, v, m);
        let rhs = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BITLSHIFT_UINT, vm, f);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::BITOR_UINT, lhs, rhs)
    };

    let num = build.inst_ir_cmd_ir_op(IrCmd::UINT_TO_NUM, value);
    let ra_reg = build.vm_reg(ra as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, ra_reg, num);

    if ra != arg {
        let ra_reg = build.vm_reg(ra as u8);
        let tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_reg, tag);
    }

    BuiltinImplResult {
        r#type: BuiltinImplType::Full,
        actual_result_count: 1,
    }
}
