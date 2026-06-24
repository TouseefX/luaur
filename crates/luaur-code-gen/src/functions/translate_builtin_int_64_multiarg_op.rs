use crate::enums::builtin_impl_type::BuiltinImplType;
use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_condition::IrCondition;
use crate::functions::builtin_check_int_64::builtin_check_int_64;
use crate::functions::builtin_load_int_64::builtin_load_int_64;
use crate::functions::vm_reg_op::vm_reg_op;
use crate::records::builtin_impl_result::BuiltinImplResult;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_builtin_int_64_multiarg_op(
    build: &mut IrBuilder,
    cmd: IrCmd,
    btest: bool,
    identity: i64,
    nparams: i32,
    ra: i32,
    arg: i32,
    args: IrOp,
    arg3: IrOp,
    nresults: i32,
    pcpos: i32,
) -> BuiltinImplResult {
    const K_INT64_BINARY_OP_UNROLLED_PARAMS: i32 = 5;

    if nparams > K_INT64_BINARY_OP_UNROLLED_PARAMS || nresults > 1 {
        return BuiltinImplResult {
            r#type: BuiltinImplType::None,
            actual_result_count: -1,
        };
    }

    if nparams == 0 {
        let vm_reg_ra = build.vm_reg(ra as u8);
        if btest {
            let const_int_1 = build.const_int(1);
            build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, vm_reg_ra, const_int_1);
            let const_tag_bool = build.const_tag(1); // LUA_TBOOLEAN
            build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, vm_reg_ra, const_tag_bool);
        } else {
            let const_int64_identity = build.const_int_64(identity);
            build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, vm_reg_ra, const_int64_identity);
            let const_tag_int = build.const_tag(lua_Type::LUA_TINTEGER as u8);
            build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, vm_reg_ra, const_tag_int);
        }
        return BuiltinImplResult {
            r#type: BuiltinImplType::Full,
            actual_result_count: 1,
        };
    }

    let vm_reg_arg = build.vm_reg(arg as u8);
    builtin_check_int_64(build, vm_reg_arg, pcpos);

    if nparams >= 2 {
        builtin_check_int_64(build, args, pcpos);
    }

    if nparams >= 3 {
        builtin_check_int_64(build, arg3, pcpos);
    }

    for i in 4..=nparams {
        let base_reg = vm_reg_op(args);
        let reg = build.vm_reg((base_reg + (i - 2)) as u8);
        builtin_check_int_64(build, reg, pcpos);
    }

    let vm_reg_arg_2 = build.vm_reg(arg as u8);
    let mut res = builtin_load_int_64(build, vm_reg_arg_2);

    if nparams >= 2 {
        let vb = builtin_load_int_64(build, args);
        res = build.inst_ir_cmd_ir_op_ir_op(cmd, res, vb);
    }

    if nparams >= 3 {
        let vc = builtin_load_int_64(build, arg3);
        res = build.inst_ir_cmd_ir_op_ir_op(cmd, res, vc);
    }

    for i in 4..=nparams {
        let base_reg = vm_reg_op(args);
        let reg = build.vm_reg((base_reg + (i - 2)) as u8);
        let vc = builtin_load_int_64(build, reg);
        res = build.inst_ir_cmd_ir_op_ir_op(cmd, res, vc);
    }

    let vm_reg_ra = build.vm_reg(ra as u8);
    if btest {
        let const_int64_0 = build.const_int_64(0);
        let cond = build.cond(IrCondition::NotEqual);
        let result =
            build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CMP_INT64, res, const_int64_0, cond);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, vm_reg_ra, result);
        let const_tag_bool = build.const_tag(1); // LUA_TBOOLEAN
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, vm_reg_ra, const_tag_bool);
    } else {
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, vm_reg_ra, res);
        let const_tag_int = build.const_tag(lua_Type::LUA_TINTEGER as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, vm_reg_ra, const_tag_int);
    }

    BuiltinImplResult {
        r#type: BuiltinImplType::Full,
        actual_result_count: 1,
    }
}
