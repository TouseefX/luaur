use core::mem::offset_of;

use crate::functions::compare_member_name::compare_member_name;
use crate::functions::type_to_userdata_index::type_to_userdata_index;
use crate::records::vec_2_conformance_ir_hooks::Vec2;
use crate::records::vertex::Vertex;
use luaur_code_gen::enums::ir_cmd::IrCmd;
use luaur_code_gen::records::ir_builder::IrBuilder;
use luaur_code_gen::records::ir_op::IrOp;
use luaur_vm::enums::lua_type::lua_Type;

const K_USERDATA_COLOR: u8 = 1;
const K_USERDATA_VEC2: u8 = 2;
const K_USERDATA_MAT3: u8 = 3;
const K_USERDATA_VERTEX: u8 = 4;
const K_TAG_VEC2: i32 = 12;
const K_TAG_VERTEX: i32 = 13;

fn check_user_data(build: &mut IrBuilder, udata: IrOp, tag: i32, pcpos: i32) {
    let tag = build.const_int(tag);
    let exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_USERDATA_TAG, udata, tag, exit);
}

fn read_user_data_f32(build: &mut IrBuilder, udata: IrOp, offset: usize) -> IrOp {
    let offset = build.const_int(offset as i32);
    let tag = build.const_tag(lua_Type::LUA_TUSERDATA as u8);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BUFFER_READF32, udata, offset, tag)
}

fn store_number(build: &mut IrBuilder, result_reg: i32, value: IrOp) {
    let result = build.vm_reg(result_reg as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, result, value);
    let result = build.vm_reg(result_reg as u8);
    let tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, result, tag);
}

fn store_userdata(build: &mut IrBuilder, result_reg: i32, value: IrOp) {
    let result = build.vm_reg(result_reg as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_POINTER, result, value);
    let result = build.vm_reg(result_reg as u8);
    let tag = build.const_tag(lua_Type::LUA_TUSERDATA as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, result, tag);
}

fn store_vector(build: &mut IrBuilder, result_reg: i32, x: IrOp, y: IrOp, z: IrOp) {
    let result = build.vm_reg(result_reg as u8);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::STORE_VECTOR, result, x, y, z);
    let result = build.vm_reg(result_reg as u8);
    let tag = build.const_tag(lua_Type::LUA_TVECTOR as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, result, tag);
}

fn write_vec2_field(build: &mut IrBuilder, udata: IrOp, offset: usize, value: IrOp) {
    let offset = build.const_int(offset as i32);
    let tag = build.const_tag(lua_Type::LUA_TUSERDATA as u8);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BUFFER_WRITEF32, udata, offset, value, tag);
}

pub fn userdata_access(
    build: &mut IrBuilder,
    r#type: u8,
    member: *const core::ffi::c_char,
    member_length: usize,
    result_reg: i32,
    source_reg: i32,
    pcpos: i32,
) -> bool {
    match type_to_userdata_index(r#type) {
        K_USERDATA_COLOR => {}
        K_USERDATA_VEC2 => {
            if compare_member_name(member, member_length, c"X".as_ptr())
                || compare_member_name(member, member_length, c"Y".as_ptr())
            {
                let source = build.vm_reg(source_reg as u8);
                let udata = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, source);
                check_user_data(build, udata, K_TAG_VEC2, pcpos);

                let field_offset = if compare_member_name(member, member_length, c"X".as_ptr()) {
                    offset_of!(Vec2, x)
                } else {
                    offset_of!(Vec2, y)
                };
                let value = read_user_data_f32(build, udata, field_offset);
                let value = build.inst_ir_cmd_ir_op(IrCmd::FLOAT_TO_NUM, value);
                store_number(build, result_reg, value);
                return true;
            }

            if compare_member_name(member, member_length, c"Magnitude".as_ptr()) {
                let source = build.vm_reg(source_reg as u8);
                let udata = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, source);
                check_user_data(build, udata, K_TAG_VEC2, pcpos);

                let x = read_user_data_f32(build, udata, offset_of!(Vec2, x));
                let y = read_user_data_f32(build, udata, offset_of!(Vec2, y));
                let x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, x, x);
                let y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, y, y);
                let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_FLOAT, x2, y2);
                let mag = build.inst_ir_cmd_ir_op(IrCmd::SQRT_FLOAT, sum);
                let mag = build.inst_ir_cmd_ir_op(IrCmd::FLOAT_TO_NUM, mag);
                store_number(build, result_reg, mag);
                return true;
            }

            if compare_member_name(member, member_length, c"Unit".as_ptr()) {
                let source = build.vm_reg(source_reg as u8);
                let udata = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, source);
                check_user_data(build, udata, K_TAG_VEC2, pcpos);

                let x = read_user_data_f32(build, udata, offset_of!(Vec2, x));
                let y = read_user_data_f32(build, udata, offset_of!(Vec2, y));
                let x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, x, x);
                let y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, y, y);
                let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_FLOAT, x2, y2);
                let mag = build.inst_ir_cmd_ir_op(IrCmd::SQRT_FLOAT, sum);
                let one = build.const_double(1.0);
                let inv = build.inst_ir_cmd_ir_op_ir_op(IrCmd::DIV_FLOAT, one, mag);
                let xr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, x, inv);
                let yr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, y, inv);

                build.inst_ir_cmd(IrCmd::CHECK_GC);
                let size = build.const_int(core::mem::size_of::<Vec2>() as i32);
                let tag = build.const_int(K_TAG_VEC2);
                let result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::NEW_USERDATA, size, tag);
                write_vec2_field(build, result, offset_of!(Vec2, x), xr);
                write_vec2_field(build, result, offset_of!(Vec2, y), yr);
                store_userdata(build, result_reg, result);
                return true;
            }
        }
        K_USERDATA_MAT3 => {}
        K_USERDATA_VERTEX => {
            if compare_member_name(member, member_length, c"pos".as_ptr())
                || compare_member_name(member, member_length, c"normal".as_ptr())
            {
                let source = build.vm_reg(source_reg as u8);
                let udata = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, source);
                check_user_data(build, udata, K_TAG_VERTEX, pcpos);

                let base = if compare_member_name(member, member_length, c"pos".as_ptr()) {
                    offset_of!(Vertex, pos)
                } else {
                    offset_of!(Vertex, normal)
                };

                let x = read_user_data_f32(build, udata, base);
                let y = read_user_data_f32(build, udata, base + core::mem::size_of::<f32>());
                let z = read_user_data_f32(build, udata, base + 2 * core::mem::size_of::<f32>());
                store_vector(build, result_reg, x, y, z);
                return true;
            }

            if compare_member_name(member, member_length, c"uv".as_ptr()) {
                let source = build.vm_reg(source_reg as u8);
                let udata = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, source);
                check_user_data(build, udata, K_TAG_VERTEX, pcpos);

                let x = read_user_data_f32(build, udata, offset_of!(Vertex, uv));
                let y = read_user_data_f32(
                    build,
                    udata,
                    offset_of!(Vertex, uv) + core::mem::size_of::<f32>(),
                );

                build.inst_ir_cmd(IrCmd::CHECK_GC);
                let size = build.const_int(core::mem::size_of::<Vec2>() as i32);
                let tag = build.const_int(K_TAG_VEC2);
                let result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::NEW_USERDATA, size, tag);
                write_vec2_field(build, result, offset_of!(Vec2, x), x);
                write_vec2_field(build, result, offset_of!(Vec2, y), y);
                store_userdata(build, result_reg, result);
                return true;
            }
        }
        _ => {}
    }

    false
}
