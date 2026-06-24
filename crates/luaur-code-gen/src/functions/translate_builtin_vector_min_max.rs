use crate::enums::builtin_impl_type::BuiltinImplType;
use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::records::builtin_impl_result::BuiltinImplResult;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_builtin_vector_min_max(
    build: &mut IrBuilder,
    cmd: IrCmd,
    nparams: i32,
    ra: i32,
    arg: i32,
    args: IrOp,
    _arg3: IrOp,
    nresults: i32,
    pcpos: i32,
) -> BuiltinImplResult {
    let arg1 = build.vm_reg(arg as u8);

    if nparams != 2
        || nresults > 1
        || arg1.kind() == IrOpKind::Constant
        || args.kind() == IrOpKind::Constant
    {
        return BuiltinImplResult {
            r#type: BuiltinImplType::None,
            actual_result_count: -1,
        };
    }

    let fallback = build.vm_exit(pcpos as u32);
    build.load_and_check_tag(arg1, lua_Type::LUA_TVECTOR as u8, fallback);
    build.load_and_check_tag(args, lua_Type::LUA_TVECTOR as u8, fallback);

    let value1 = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, arg1);
    let value2 = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, args);

    let ret = build.inst_ir_cmd_ir_op_ir_op(cmd, value2, value1);

    let ra_reg = build.vm_reg(ra as u8);
    let tag_vector = build.inst_ir_cmd_ir_op(IrCmd::TAG_VECTOR, ret);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, ra_reg, tag_vector);

    BuiltinImplResult {
        r#type: BuiltinImplType::Full,
        actual_result_count: 1,
    }
}
