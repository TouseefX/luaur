use crate::enums::builtin_impl_type::BuiltinImplType;
use crate::enums::int_64_binary::Int64Binary;
use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_condition::IrCondition;
use crate::functions::builtin_check_int_64::builtin_check_int_64;
use crate::functions::builtin_load_int_64::builtin_load_int_64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::builtin_impl_result::BuiltinImplResult;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_builtin_int_64_binary(
    build: &mut IrBuilder,
    nparams: i32,
    ra: i32,
    arg: i32,
    args: IrOp,
    nresults: i32,
    pcpos: i32,
    op: Int64Binary,
) -> BuiltinImplResult {
    if nparams < 2 || nresults > 1 {
        return BuiltinImplResult {
            r#type: BuiltinImplType::None,
            actual_result_count: -1,
        };
    }

    let vm_reg_arg = build.vm_reg(arg as u8);
    builtin_check_int_64(build, vm_reg_arg, pcpos);
    builtin_check_int_64(build, args, pcpos);

    let va = builtin_load_int_64(build, vm_reg_arg);
    let vb = builtin_load_int_64(build, args);

    let bin_op = match op {
        Int64Binary::Add => build.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_INT64, va, vb),
        Int64Binary::Sub => build.inst_ir_cmd_ir_op_ir_op(IrCmd::SUB_INT64, va, vb),
        Int64Binary::Mul => build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_INT64, va, vb),
        Int64Binary::Div => {
            let exit = build.vm_exit(pcpos as u32);
            build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_DIV_INT64, va, vb, exit);
            build.inst_ir_cmd_ir_op_ir_op(IrCmd::DIV_INT64, va, vb)
        }
        Int64Binary::Idiv => {
            let exit = build.vm_exit(pcpos as u32);
            build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_DIV_INT64, va, vb, exit);
            build.inst_ir_cmd_ir_op_ir_op(IrCmd::IDIV_INT64, va, vb)
        }
        Int64Binary::Udiv => {
            let zero = build.const_int_64(0);
            let cond = build.cond(IrCondition::NotEqual);
            let exit = build.vm_exit(pcpos as u32);
            build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CHECK_CMP_INT64, vb, zero, cond, exit);
            build.inst_ir_cmd_ir_op_ir_op(IrCmd::UDIV_INT64, va, vb)
        }
        Int64Binary::Rem => {
            let zero = build.const_int_64(0);
            let cond = build.cond(IrCondition::NotEqual);
            let exit = build.vm_exit(pcpos as u32);
            build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CHECK_CMP_INT64, vb, zero, cond, exit);
            build.inst_ir_cmd_ir_op_ir_op(IrCmd::REM_INT64, va, vb)
        }
        Int64Binary::Urem => {
            let zero = build.const_int_64(0);
            let cond = build.cond(IrCondition::NotEqual);
            let exit = build.vm_exit(pcpos as u32);
            build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CHECK_CMP_INT64, vb, zero, cond, exit);
            build.inst_ir_cmd_ir_op_ir_op(IrCmd::UREM_INT64, va, vb)
        }
        Int64Binary::Mod => {
            let zero = build.const_int_64(0);
            let cond = build.cond(IrCondition::NotEqual);
            let exit = build.vm_exit(pcpos as u32);
            build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CHECK_CMP_INT64, vb, zero, cond, exit);
            build.inst_ir_cmd_ir_op_ir_op(IrCmd::MOD_INT64, va, vb)
        }
    };

    let vm_reg_ra = build.vm_reg(ra as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, vm_reg_ra, bin_op);
    let tag = build.const_tag(lua_Type::LUA_TINTEGER as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, vm_reg_ra, tag);

    BuiltinImplResult {
        r#type: BuiltinImplType::Full,
        actual_result_count: 1,
    }
}
