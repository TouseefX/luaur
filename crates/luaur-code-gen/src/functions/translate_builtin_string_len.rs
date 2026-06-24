use crate::enums::builtin_impl_type::BuiltinImplType;
use crate::enums::ir_cmd::IrCmd;
use crate::records::builtin_impl_result::BuiltinImplResult;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_builtin_string_len(
    build: &mut IrBuilder,
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

    let vm_reg_arg = build.vm_reg(arg as u8);
    let vm_exit = build.vm_exit(pcpos as u32);
    build.load_and_check_tag(vm_reg_arg, lua_Type::LUA_TSTRING as u8, vm_exit);

    let vm_reg_arg_for_load = build.vm_reg(arg as u8);
    let ts = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, vm_reg_arg_for_load);
    let len = build.inst_ir_cmd_ir_op(IrCmd::STRING_LEN, ts);

    let ra_reg = build.vm_reg(ra as u8);
    let len_num = build.inst_ir_cmd_ir_op(IrCmd::INT_TO_NUM, len);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, ra_reg, len_num);
    let tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_reg, tag);

    BuiltinImplResult {
        r#type: BuiltinImplType::Full,
        actual_result_count: 1,
    }
}
