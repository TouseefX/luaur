use crate::enums::builtin_impl_type::BuiltinImplType;
use crate::enums::ir_cmd::IrCmd;
use crate::functions::translate_buffer_args_and_check_bounds::translate_buffer_args_and_check_bounds;
use crate::records::builtin_impl_result::BuiltinImplResult;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_builtin_buffer_read(
    build: &mut IrBuilder,
    nparams: i32,
    ra: i32,
    arg: i32,
    args: IrOp,
    arg3: IrOp,
    nresults: i32,
    pcpos: i32,
    read_cmd: IrCmd,
    size: i32,
    conv_cmd: IrCmd,
    store_cmd: IrCmd,
    store_tag: u8,
) -> BuiltinImplResult {
    if nparams < 2 || nresults > 1 {
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
        false,
    );

    let tag_buffer = build.const_tag(lua_Type::LUA_TBUFFER as u8);
    let result = build.inst_ir_cmd_ir_op_ir_op_ir_op(read_cmd, buf, int_index, tag_buffer);

    let value_to_store = if conv_cmd == IrCmd::NOP {
        result
    } else {
        build.inst_ir_cmd_ir_op(conv_cmd, result)
    };

    let ra_vm_reg = build.vm_reg(ra as u8);
    build.inst_ir_cmd_ir_op_ir_op(store_cmd, ra_vm_reg, value_to_store);
    let tag = build.const_tag(store_tag);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_vm_reg, tag);

    BuiltinImplResult {
        r#type: BuiltinImplType::Full,
        actual_result_count: 1,
    }
}
