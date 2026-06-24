use crate::enums::ir_cmd::IrCmd;
use crate::functions::builtin_check_double::builtin_check_double;
use crate::functions::builtin_check_int_64::builtin_check_int_64;
use crate::functions::builtin_load_double::builtin_load_double;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_buffer_args_and_check_bounds(
    build: &mut IrBuilder,
    nparams: i32,
    arg: i32,
    args: IrOp,
    arg3: IrOp,
    size: i32,
    pcpos: i32,
    buf: &mut IrOp,
    int_index: &mut IrOp,
    load_int_64: bool,
) {
    let loc = build.vm_reg(arg as u8);
    let fallback = build.vm_exit(pcpos as u32);
    build.load_and_check_tag(loc, lua_Type::LUA_TBUFFER as u8, fallback);

    builtin_check_double(build, args, pcpos);

    if nparams == 3 && load_int_64 {
        builtin_check_int_64(build, arg3, pcpos);
    } else if nparams == 3 {
        builtin_check_double(build, arg3, pcpos);
    }

    let reg_arg = build.vm_reg(arg as u8);
    *buf = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, reg_arg);

    let num_index = builtin_load_double(build, args);
    *int_index = build.inst_ir_cmd_ir_op(IrCmd::NUM_TO_INT, num_index);

    let buf_op = *buf;
    let int_index_op = *int_index;
    let zero = build.const_int(0);
    let size_op = build.const_int(size);
    let undef_op = build.undef();
    let exit_op = build.vm_exit(pcpos as u32);

    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::CHECK_BUFFER_LEN,
        buf_op,
        int_index_op,
        zero,
        size_op,
        undef_op,
        exit_op,
    );
}
