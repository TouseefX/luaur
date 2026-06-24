use crate::enums::builtin_impl_type::BuiltinImplType;
use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::records::builtin_impl_result::BuiltinImplResult;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_builtin_vector_magnitude(
    build: &mut IrBuilder,
    nparams: i32,
    ra: i32,
    arg: i32,
    _args: IrOp,
    _arg3: IrOp,
    nresults: i32,
    pcpos: i32,
) -> BuiltinImplResult {
    let arg1 = build.vm_reg(arg as u8);

    if nparams != 1 || nresults > 1 || arg1.kind() == IrOpKind::Constant {
        return BuiltinImplResult {
            r#type: BuiltinImplType::None,
            actual_result_count: -1,
        };
    }

    let fallback = build.vm_exit(pcpos as u32);
    let tag_vector = lua_Type::LUA_TVECTOR as u8;
    build.load_and_check_tag(arg1, tag_vector, fallback);

    let zero = build.const_int(0);
    let a = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_TVALUE, arg1, zero);

    let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::DOT_VEC, a, a);
    let mag = build.inst_ir_cmd_ir_op(IrCmd::SQRT_FLOAT, sum);

    let mag_num = build.inst_ir_cmd_ir_op(IrCmd::FLOAT_TO_NUM, mag);

    let ra_reg = build.vm_reg(ra as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, ra_reg, mag_num);
    let tag_number = build.const_tag(lua_Type::LUA_TNUMBER as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_reg, tag_number);

    BuiltinImplResult {
        r#type: BuiltinImplType::Full,
        actual_result_count: 1,
    }
}
