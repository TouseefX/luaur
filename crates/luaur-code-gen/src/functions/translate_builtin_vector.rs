use crate::enums::builtin_impl_type::BuiltinImplType;
use crate::enums::ir_cmd::IrCmd;
use crate::functions::builtin_check_double::builtin_check_double;
use crate::functions::builtin_load_double::builtin_load_double;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::builtin_impl_result::BuiltinImplResult;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_vm::macros::lua_vector_size::LUA_VECTOR_SIZE;

pub fn translate_builtin_vector(
    build: &mut IrBuilder,
    nparams: i32,
    ra: i32,
    arg: i32,
    args: IrOp,
    arg3: IrOp,
    nresults: i32,
    pcpos: i32,
) -> BuiltinImplResult {
    if nparams < 2 || nresults > 1 {
        return BuiltinImplResult {
            r#type: BuiltinImplType::None,
            actual_result_count: -1,
        };
    }

    CODEGEN_ASSERT!(LUA_VECTOR_SIZE == 3);

    let ra_op = build.vm_reg(ra as u8);

    if nparams == 2 {
        let arg_op = build.vm_reg(arg as u8);
        builtin_check_double(build, arg_op, pcpos);
        builtin_check_double(build, args, pcpos);

        let x = builtin_load_double(build, arg_op);
        let y = builtin_load_double(build, args);

        let xf = build.inst_ir_cmd_ir_op(IrCmd::NUM_TO_FLOAT, x);
        let yf = build.inst_ir_cmd_ir_op(IrCmd::NUM_TO_FLOAT, y);

        let zero = build.const_double(0.0);
        build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::STORE_VECTOR, ra_op, xf, yf, zero);
    } else {
        let arg_op = build.vm_reg(arg as u8);
        builtin_check_double(build, arg_op, pcpos);
        builtin_check_double(build, args, pcpos);
        builtin_check_double(build, arg3, pcpos);

        let x = builtin_load_double(build, arg_op);
        let y = builtin_load_double(build, args);
        let z = builtin_load_double(build, arg3);

        let xf = build.inst_ir_cmd_ir_op(IrCmd::NUM_TO_FLOAT, x);
        let yf = build.inst_ir_cmd_ir_op(IrCmd::NUM_TO_FLOAT, y);
        let zf = build.inst_ir_cmd_ir_op(IrCmd::NUM_TO_FLOAT, z);

        build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::STORE_VECTOR, ra_op, xf, yf, zf);
    }

    let tag = build.const_tag(luaur_vm::enums::lua_type::lua_Type::LUA_TVECTOR as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_op, tag);

    BuiltinImplResult {
        r#type: BuiltinImplType::Full,
        actual_result_count: 1,
    }
}
