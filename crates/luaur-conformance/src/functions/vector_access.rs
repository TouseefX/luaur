use crate::functions::compare_member_name::compare_member_name;
use luaur_code_gen::enums::ir_cmd::IrCmd;
use luaur_code_gen::records::ir_builder::IrBuilder;
use luaur_vm::enums::lua_type::lua_Type;

pub fn vector_access(
    build: &mut IrBuilder,
    member: *const core::ffi::c_char,
    member_length: usize,
    result_reg: i32,
    source_reg: i32,
    _pcpos: i32,
) -> bool {
    if compare_member_name(member, member_length, c"Magnitude".as_ptr()) {
        let vm_reg = build.vm_reg(source_reg as u8);
        let c0 = build.const_int(0);
        let c4 = build.const_int(4);
        let c8 = build.const_int(8);
        let x = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, vm_reg, c0);
        let y = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, vm_reg, c4);
        let z = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, vm_reg, c8);

        let x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, x, x);
        let y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, y, y);
        let z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, z, z);

        let sum_xy = build.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_FLOAT, x2, y2);
        let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_FLOAT, sum_xy, z2);

        let mag = build.inst_ir_cmd_ir_op(IrCmd::SQRT_FLOAT, sum);
        let mag_num = build.inst_ir_cmd_ir_op(IrCmd::FLOAT_TO_NUM, mag);

        let res_reg = build.vm_reg(result_reg as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, res_reg, mag_num);
        let tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, res_reg, tag);

        return true;
    }

    if compare_member_name(member, member_length, c"Unit".as_ptr()) {
        let vm_reg = build.vm_reg(source_reg as u8);
        let c0 = build.const_int(0);
        let c4 = build.const_int(4);
        let c8 = build.const_int(8);
        let x = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, vm_reg, c0);
        let y = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, vm_reg, c4);
        let z = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, vm_reg, c8);

        let x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, x, x);
        let y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, y, y);
        let z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, z, z);

        let sum_xy = build.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_FLOAT, x2, y2);
        let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_FLOAT, sum_xy, z2);

        let mag = build.inst_ir_cmd_ir_op(IrCmd::SQRT_FLOAT, sum);
        let one = build.const_double(1.0);
        let inv = build.inst_ir_cmd_ir_op_ir_op(IrCmd::DIV_FLOAT, one, mag);

        let xr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, x, inv);
        let yr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, y, inv);
        let zr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, z, inv);

        let res_reg = build.vm_reg(result_reg as u8);
        build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::STORE_VECTOR, res_reg, xr, yr, zr);
        let tag = build.const_tag(lua_Type::LUA_TVECTOR as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, res_reg, tag);

        return true;
    }

    false
}
