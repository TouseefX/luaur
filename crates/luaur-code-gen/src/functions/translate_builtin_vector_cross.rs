use crate::enums::builtin_impl_type::BuiltinImplType;
use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::records::builtin_impl_result::BuiltinImplResult;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_builtin_vector_cross(
    build: &mut IrBuilder,
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
    let tag_vector = lua_Type::LUA_TVECTOR as u8;
    build.load_and_check_tag(arg1, tag_vector, fallback);
    build.load_and_check_tag(args, tag_vector, fallback);

    let c0 = build.const_int(0);
    let c4 = build.const_int(4);
    let c8 = build.const_int(8);

    let x1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, arg1, c0);
    let x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, args, c0);

    let y1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, arg1, c4);
    let y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, args, c4);

    let z1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, arg1, c8);
    let z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, args, c8);

    let y1z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, y1, z2);
    let z1y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, z1, y2);
    let xr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SUB_FLOAT, y1z2, z1y2);

    let z1x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, z1, x2);
    let x1z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, x1, z2);
    let yr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SUB_FLOAT, z1x2, x1z2);

    let x1y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, x1, y2);
    let y1x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, y1, x2);
    let zr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SUB_FLOAT, x1y2, y1x2);

    let ra_reg = build.vm_reg(ra as u8);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::STORE_VECTOR, ra_reg, xr, yr, zr);
    let tag_vector_const = build.const_tag(tag_vector);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_reg, tag_vector_const);

    BuiltinImplResult {
        r#type: BuiltinImplType::Full,
        actual_result_count: 1,
    }
}
