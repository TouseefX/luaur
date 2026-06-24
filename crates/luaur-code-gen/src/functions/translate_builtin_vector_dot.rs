use crate::enums::builtin_impl_type::BuiltinImplType;
use crate::enums::ir_cmd::IrCmd;
use crate::records::builtin_impl_result::BuiltinImplResult;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_builtin_vector_dot(
    build: &mut IrBuilder,
    nparams: i32,
    ra: i32,
    arg: i32,
    args: IrOp,
    arg3: IrOp,
    nresults: i32,
    pcpos: i32,
) -> BuiltinImplResult {
    let arg1 = build.vm_reg(arg as u8);

    if nparams != 2
        || nresults > 1
        || arg1.kind() == crate::enums::ir_op_kind::IrOpKind::Constant
        || args.kind() == crate::enums::ir_op_kind::IrOpKind::Constant
    {
        return BuiltinImplResult {
            r#type: BuiltinImplType::None,
            actual_result_count: -1,
        };
    }

    let fallback = build.vm_exit(pcpos as u32);
    build.load_and_check_tag(arg1, lua_Type::LUA_TVECTOR as u8, fallback);
    build.load_and_check_tag(args, lua_Type::LUA_TVECTOR as u8, fallback);

    let a = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, arg1);
    let b = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, args);

    let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::DOT_VEC, a, b);
    let sum = build.inst_ir_cmd_ir_op(IrCmd::FLOAT_TO_NUM, sum);

    let ra_reg = build.vm_reg(ra as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, ra_reg, sum);
    let tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_reg, tag);

    let _ = arg3;

    BuiltinImplResult {
        r#type: BuiltinImplType::Full,
        actual_result_count: 1,
    }
}
