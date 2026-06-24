use crate::enums::builtin_impl_type::BuiltinImplType;
use crate::enums::ir_block_kind::IrBlockKind;
use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_condition::IrCondition;
use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::builtin_check_double::builtin_check_double;
use crate::functions::builtin_load_double::builtin_load_double;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::builtin_impl_result::BuiltinImplResult;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_builtin_math_clamp(
    build: &mut IrBuilder,
    nparams: i32,
    ra: i32,
    arg: i32,
    args: IrOp,
    arg3: IrOp,
    nresults: i32,
    fallback: IrOp,
    pcpos: i32,
) -> BuiltinImplResult {
    if nparams < 3 || nresults > 1 {
        return BuiltinImplResult {
            r#type: BuiltinImplType::None,
            actual_result_count: -1,
        };
    }

    let block = build.block(IrBlockKind::Internal);
    CODEGEN_ASSERT!(args.kind() == IrOpKind::VmReg);

    let arg_reg = build.vm_reg(arg as u8);
    builtin_check_double(build, arg_reg, pcpos);
    builtin_check_double(build, args, pcpos);
    builtin_check_double(build, arg3, pcpos);

    let min = builtin_load_double(build, args);
    let max = builtin_load_double(build, arg3);
    let cond = build.cond(IrCondition::NotLessEqual);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JUMP_CMP_NUM,
        min,
        max,
        cond,
        fallback,
        block,
    );
    build.begin_block(block);

    let arg_reg = build.vm_reg(arg as u8);
    let v = builtin_load_double(build, arg_reg);
    let r = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MAX_NUM, min, v);
    let clamped = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MIN_NUM, max, r);

    let ra_reg = build.vm_reg(ra as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, ra_reg, clamped);

    if ra != arg {
        let ra_reg = build.vm_reg(ra as u8);
        let tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_reg, tag);
    }

    BuiltinImplResult {
        r#type: BuiltinImplType::UsesFallback,
        actual_result_count: 1,
    }
}
