use crate::enums::builtin_impl_type::BuiltinImplType;
use crate::enums::ir_cmd::IrCmd;
use crate::records::builtin_impl_result::BuiltinImplResult;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_builtin_typeof(
    build: &mut IrBuilder,
    nparams: i32,
    ra: i32,
    arg: i32,
    _args: IrOp,
    nresults: i32,
) -> BuiltinImplResult {
    if nparams < 1 || nresults > 1 {
        return BuiltinImplResult {
            r#type: BuiltinImplType::None,
            actual_result_count: -1,
        };
    }

    let vm_reg_arg = build.vm_reg(arg as u8);
    let name = build.inst_ir_cmd_ir_op(IrCmd::GET_TYPEOF, vm_reg_arg);

    let vm_reg_ra = build.vm_reg(ra as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_POINTER, vm_reg_ra, name);

    let tag = build.const_tag(lua_Type::LUA_TSTRING as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, vm_reg_ra, tag);

    BuiltinImplResult {
        r#type: BuiltinImplType::Full,
        actual_result_count: 1,
    }
}
