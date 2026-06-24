use crate::enums::builtin_impl_type::BuiltinImplType;
use crate::enums::ir_cmd::IrCmd;
use crate::functions::builtin_check_double::builtin_check_double;
use crate::functions::builtin_load_double::builtin_load_double;
use crate::records::builtin_impl_result::BuiltinImplResult;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_builtin_bit_32_extract_k(
    build: &mut IrBuilder,
    nparams: i32,
    ra: i32,
    arg: i32,
    args: IrOp,
    nresults: i32,
    pcpos: i32,
) -> BuiltinImplResult {
    if nparams < 2 || nresults > 1 {
        return BuiltinImplResult {
            r#type: BuiltinImplType::None,
            actual_result_count: -1,
        };
    }

    let arg_reg = build.vm_reg(arg as u8);
    builtin_check_double(build, arg_reg, pcpos);

    let va = builtin_load_double(build, arg_reg);
    let n = build.inst_ir_cmd_ir_op(IrCmd::NUM_TO_UINT, va);

    let a2 = unsafe { build.function.double_op(args) };
    let fw = a2 as i32;

    let f = fw & 31;
    let w1 = fw >> 5;

    let m = !(0xfffffffeu32 << w1) as u32;

    let mut result = n;

    if f != 0 {
        let shift_op = build.const_int(f);
        result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BITRSHIFT_UINT, result, shift_op);
    }

    if (f + w1 + 1) < 32 {
        let mask_op = build.const_int(m as i32);
        result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BITAND_UINT, result, mask_op);
    }

    let value = build.inst_ir_cmd_ir_op(IrCmd::UINT_TO_NUM, result);
    let ra_reg = build.vm_reg(ra as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, ra_reg, value);

    if ra != arg {
        let tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_reg, tag);
    }

    BuiltinImplResult {
        r#type: BuiltinImplType::Full,
        actual_result_count: 1,
    }
}
