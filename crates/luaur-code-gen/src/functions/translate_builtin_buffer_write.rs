use crate::enums::builtin_impl_type::BuiltinImplType;
use crate::enums::ir_cmd::IrCmd;
use crate::functions::builtin_load_double::builtin_load_double;
use crate::functions::builtin_load_int_64::builtin_load_int_64;
use crate::functions::translate_buffer_args_and_check_bounds::translate_buffer_args_and_check_bounds;
use crate::records::builtin_impl_result::BuiltinImplResult;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_builtin_buffer_write(
    build: &mut IrBuilder,
    nparams: i32,
    _ra: i32,
    arg: i32,
    args: IrOp,
    arg3: IrOp,
    nresults: i32,
    pcpos: i32,
    write_cmd: IrCmd,
    size: i32,
    conv_cmd: IrCmd,
    load_int_64: bool,
) -> BuiltinImplResult {
    if nparams < 3 || nresults > 0 {
        return BuiltinImplResult {
            r#type: BuiltinImplType::None,
            actual_result_count: -1,
        };
    }

    let mut buf = IrOp::ir_op();
    let mut int_index = IrOp::ir_op();

    translate_buffer_args_and_check_bounds(
        build,
        nparams,
        arg,
        args,
        arg3,
        size,
        pcpos,
        &mut buf,
        &mut int_index,
        load_int_64,
    );

    let num_value = if load_int_64 {
        builtin_load_int_64(build, arg3)
    } else {
        builtin_load_double(build, arg3)
    };

    let value_to_write = if conv_cmd == IrCmd::NOP {
        num_value
    } else {
        build.inst_ir_cmd_ir_op(conv_cmd, num_value)
    };

    let tag_buffer = build.const_tag(lua_Type::LUA_TBUFFER as u8);

    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        write_cmd,
        buf,
        int_index,
        value_to_write,
        tag_buffer,
    );

    BuiltinImplResult {
        r#type: BuiltinImplType::Full,
        actual_result_count: 0,
    }
}
