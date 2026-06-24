use crate::enums::builtin_impl_type::BuiltinImplType;
use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_condition::IrCondition;
use crate::functions::builtin_check_double::builtin_check_double;
use crate::functions::builtin_load_double::builtin_load_double;
use crate::records::builtin_impl_result::BuiltinImplResult;
use crate::records::ir_builder::IrBuilder;

pub fn translate_builtin_int_64_create(
    build: &mut IrBuilder,
    nparams: i32,
    ra: i32,
    arg: i32,
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

    let arg_value = builtin_load_double(build, arg_reg);

    let integer_value = build.inst_ir_cmd_ir_op(IrCmd::NUM_TO_INT64, arg_value);
    let back_to_double = build.inst_ir_cmd_ir_op(IrCmd::INT64_TO_NUM, integer_value);

    let cond = build.cond(IrCondition::Equal);
    let exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::CHECK_CMP_NUM,
        back_to_double,
        arg_value,
        cond,
        exit,
    );

    let ra_reg = build.vm_reg(ra as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, ra_reg, integer_value);

    let tag = build.const_tag(luaur_vm::enums::lua_type::lua_Type::LUA_TINTEGER as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_reg, tag);

    BuiltinImplResult {
        r#type: BuiltinImplType::Full,
        actual_result_count: 1,
    }
}
