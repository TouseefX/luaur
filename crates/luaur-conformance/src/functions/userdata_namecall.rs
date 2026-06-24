use core::mem::offset_of;

use crate::functions::compare_member_name::compare_member_name;
use crate::functions::type_to_userdata_index::type_to_userdata_index;
use crate::records::vec_2_conformance_ir_hooks::Vec2;
use luaur_code_gen::enums::ir_cmd::IrCmd;
use luaur_code_gen::records::ir_builder::IrBuilder;
use luaur_code_gen::records::ir_op::IrOp;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::macros::lua_multret::LUA_MULTRET;

const K_USERDATA_COLOR: u8 = 1;
const K_USERDATA_VEC2: u8 = 2;
const K_USERDATA_MAT3: u8 = 3;
const K_TAG_VEC2: i32 = 12;

fn check_user_data(build: &mut IrBuilder, udata: IrOp, pcpos: i32) {
    let tag = build.const_int(K_TAG_VEC2);
    let exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_USERDATA_TAG, udata, tag, exit);
}

fn read_user_data_f32(build: &mut IrBuilder, udata: IrOp, offset: usize) -> IrOp {
    let offset = build.const_int(offset as i32);
    let tag = build.const_tag(lua_Type::LUA_TUSERDATA as u8);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BUFFER_READF32, udata, offset, tag)
}

fn write_vec2_field(build: &mut IrBuilder, udata: IrOp, offset: usize, value: IrOp) {
    let offset = build.const_int(offset as i32);
    let tag = build.const_tag(lua_Type::LUA_TUSERDATA as u8);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BUFFER_WRITEF32, udata, offset, value, tag);
}

fn adjust_multret(build: &mut IrBuilder, arg_res_reg: i32, results: i32) {
    if results == LUA_MULTRET {
        let reg = build.vm_reg(arg_res_reg as u8);
        let count = build.const_int(1);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::ADJUST_STACK_TO_REG, reg, count);
    }
}

pub fn userdata_namecall(
    build: &mut IrBuilder,
    r#type: u8,
    member: *const core::ffi::c_char,
    member_length: usize,
    arg_res_reg: i32,
    source_reg: i32,
    _params: i32,
    results: i32,
    pcpos: i32,
) -> bool {
    match type_to_userdata_index(r#type) {
        K_USERDATA_COLOR => {}
        K_USERDATA_VEC2 => {
            if compare_member_name(member, member_length, c"Dot".as_ptr()) {
                let source = build.vm_reg(source_reg as u8);
                let udata1 = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, source);
                check_user_data(build, udata1, pcpos);

                let arg = build.vm_reg((arg_res_reg + 2) as u8);
                let exit = build.vm_exit(pcpos as u32);
                build.load_and_check_tag(arg, lua_Type::LUA_TUSERDATA as u8, exit);

                let arg = build.vm_reg((arg_res_reg + 2) as u8);
                let udata2 = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, arg);
                check_user_data(build, udata2, pcpos);

                let mut x1 = read_user_data_f32(build, udata1, offset_of!(Vec2, x));
                let mut x2 = read_user_data_f32(build, udata2, offset_of!(Vec2, x));
                x1 = build.inst_ir_cmd_ir_op(IrCmd::FLOAT_TO_NUM, x1);
                x2 = build.inst_ir_cmd_ir_op(IrCmd::FLOAT_TO_NUM, x2);
                let xx = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_NUM, x1, x2);

                let mut y1 = read_user_data_f32(build, udata1, offset_of!(Vec2, y));
                let mut y2 = read_user_data_f32(build, udata2, offset_of!(Vec2, y));
                y1 = build.inst_ir_cmd_ir_op(IrCmd::FLOAT_TO_NUM, y1);
                y2 = build.inst_ir_cmd_ir_op(IrCmd::FLOAT_TO_NUM, y2);
                let yy = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_NUM, y1, y2);
                let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_NUM, xx, yy);

                let result = build.vm_reg(arg_res_reg as u8);
                build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, result, sum);
                let result = build.vm_reg(arg_res_reg as u8);
                let tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
                build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, result, tag);
                adjust_multret(build, arg_res_reg, results);
                return true;
            }

            if compare_member_name(member, member_length, c"Min".as_ptr()) {
                let source = build.vm_reg(source_reg as u8);
                let udata1 = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, source);
                check_user_data(build, udata1, pcpos);

                let arg = build.vm_reg((arg_res_reg + 2) as u8);
                let exit = build.vm_exit(pcpos as u32);
                build.load_and_check_tag(arg, lua_Type::LUA_TUSERDATA as u8, exit);

                let arg = build.vm_reg((arg_res_reg + 2) as u8);
                let udata2 = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, arg);
                check_user_data(build, udata2, pcpos);

                let mut x1 = read_user_data_f32(build, udata1, offset_of!(Vec2, x));
                let mut x2 = read_user_data_f32(build, udata2, offset_of!(Vec2, x));
                x1 = build.inst_ir_cmd_ir_op(IrCmd::FLOAT_TO_NUM, x1);
                x2 = build.inst_ir_cmd_ir_op(IrCmd::FLOAT_TO_NUM, x2);
                let mx = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MIN_NUM, x1, x2);

                let mut y1 = read_user_data_f32(build, udata1, offset_of!(Vec2, y));
                let mut y2 = read_user_data_f32(build, udata2, offset_of!(Vec2, y));
                y1 = build.inst_ir_cmd_ir_op(IrCmd::FLOAT_TO_NUM, y1);
                y2 = build.inst_ir_cmd_ir_op(IrCmd::FLOAT_TO_NUM, y2);
                let my = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MIN_NUM, y1, y2);

                let mx = build.inst_ir_cmd_ir_op(IrCmd::NUM_TO_FLOAT, mx);
                let my = build.inst_ir_cmd_ir_op(IrCmd::NUM_TO_FLOAT, my);

                build.inst_ir_cmd(IrCmd::CHECK_GC);
                let size = build.const_int(core::mem::size_of::<Vec2>() as i32);
                let tag = build.const_int(K_TAG_VEC2);
                let udata_result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::NEW_USERDATA, size, tag);
                write_vec2_field(build, udata_result, offset_of!(Vec2, x), mx);
                write_vec2_field(build, udata_result, offset_of!(Vec2, y), my);

                let result = build.vm_reg(arg_res_reg as u8);
                build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_POINTER, result, udata_result);
                let result = build.vm_reg(arg_res_reg as u8);
                let tag = build.const_tag(lua_Type::LUA_TUSERDATA as u8);
                build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, result, tag);
                adjust_multret(build, arg_res_reg, results);
                return true;
            }
        }
        K_USERDATA_MAT3 => {}
        _ => {}
    }

    false
}
