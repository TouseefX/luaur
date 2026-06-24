use crate::functions::type_to_userdata_index::type_to_userdata_index;
use core::mem::offset_of;
use luaur_code_gen::enums::host_metamethod::HostMetamethod;
use luaur_code_gen::enums::ir_cmd::IrCmd;
use luaur_code_gen::records::ir_builder::IrBuilder;
use luaur_code_gen::records::ir_op::IrOp;

const LUA_TUSERDATA: u8 = 9;
const K_USERDATA_VEC2: u8 = 2;
const K_TAG_VEC2: i32 = 12;

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}

pub fn userdata_metamethod(
    build: &mut IrBuilder,
    lhs_ty: u8,
    rhs_ty: u8,
    result_reg: i32,
    lhs: IrOp,
    rhs: IrOp,
    method: HostMetamethod,
    pcpos: i32,
) -> bool {
    match method {
        HostMetamethod::Add | HostMetamethod::Mul => {
            if type_to_userdata_index(lhs_ty) == K_USERDATA_VEC2
                && type_to_userdata_index(rhs_ty) == K_USERDATA_VEC2
            {
                let vm_exit = build.vm_exit(pcpos as u32);
                build.load_and_check_tag(lhs, LUA_TUSERDATA, vm_exit);
                let vm_exit = build.vm_exit(pcpos as u32);
                build.load_and_check_tag(rhs, LUA_TUSERDATA, vm_exit);

                let udata1 = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, lhs);
                let tag_vec2 = build.const_int(K_TAG_VEC2);
                let vm_exit = build.vm_exit(pcpos as u32);
                build.inst_ir_cmd_ir_op_ir_op_ir_op(
                    IrCmd::CHECK_USERDATA_TAG,
                    udata1,
                    tag_vec2,
                    vm_exit,
                );

                let udata2 = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, rhs);
                let tag_vec2 = build.const_int(K_TAG_VEC2);
                let vm_exit = build.vm_exit(pcpos as u32);
                build.inst_ir_cmd_ir_op_ir_op_ir_op(
                    IrCmd::CHECK_USERDATA_TAG,
                    udata2,
                    tag_vec2,
                    vm_exit,
                );

                let off_x = build.const_int(offset_of!(Vec2, x) as i32);
                let tag_udata = build.const_tag(LUA_TUSERDATA);
                let x1 = build.inst_ir_cmd_ir_op_ir_op_ir_op(
                    IrCmd::BUFFER_READF32,
                    udata1,
                    off_x,
                    tag_udata,
                );
                let x2 = build.inst_ir_cmd_ir_op_ir_op_ir_op(
                    IrCmd::BUFFER_READF32,
                    udata2,
                    off_x,
                    tag_udata,
                );

                let cmd = if method == HostMetamethod::Add {
                    IrCmd::ADD_FLOAT
                } else {
                    IrCmd::MUL_FLOAT
                };
                let mx = build.inst_ir_cmd_ir_op_ir_op(cmd, x1, x2);

                let off_y = build.const_int(offset_of!(Vec2, y) as i32);
                let tag_udata = build.const_tag(LUA_TUSERDATA);
                let y1 = build.inst_ir_cmd_ir_op_ir_op_ir_op(
                    IrCmd::BUFFER_READF32,
                    udata1,
                    off_y,
                    tag_udata,
                );
                let y2 = build.inst_ir_cmd_ir_op_ir_op_ir_op(
                    IrCmd::BUFFER_READF32,
                    udata2,
                    off_y,
                    tag_udata,
                );

                let my = build.inst_ir_cmd_ir_op_ir_op(cmd, y1, y2);

                build.inst_ir_cmd(IrCmd::CHECK_GC);
                let size = build.const_int(core::mem::size_of::<Vec2>() as i32);
                let tag_vec2 = build.const_int(K_TAG_VEC2);
                let udatar = build.inst_ir_cmd_ir_op_ir_op(IrCmd::NEW_USERDATA, size, tag_vec2);

                let tag_udata = build.const_tag(LUA_TUSERDATA);
                build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
                    IrCmd::BUFFER_WRITEF32,
                    udatar,
                    off_x,
                    mx,
                    tag_udata,
                );
                build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
                    IrCmd::BUFFER_WRITEF32,
                    udatar,
                    off_y,
                    my,
                    tag_udata,
                );

                let vm_reg = build.vm_reg(result_reg as u8);
                build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_POINTER, vm_reg, udatar);
                let tag_udata = build.const_tag(LUA_TUSERDATA);
                build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, vm_reg, tag_udata);

                return true;
            }
        }
        HostMetamethod::Minus => {
            if type_to_userdata_index(lhs_ty) == K_USERDATA_VEC2 {
                let vm_exit = build.vm_exit(pcpos as u32);
                build.load_and_check_tag(lhs, LUA_TUSERDATA, vm_exit);

                let udata1 = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, lhs);
                let tag_vec2 = build.const_int(K_TAG_VEC2);
                let vm_exit = build.vm_exit(pcpos as u32);
                build.inst_ir_cmd_ir_op_ir_op_ir_op(
                    IrCmd::CHECK_USERDATA_TAG,
                    udata1,
                    tag_vec2,
                    vm_exit,
                );

                let off_x = build.const_int(offset_of!(Vec2, x) as i32);
                let off_y = build.const_int(offset_of!(Vec2, y) as i32);
                let tag_udata = build.const_tag(LUA_TUSERDATA);
                let x = build.inst_ir_cmd_ir_op_ir_op_ir_op(
                    IrCmd::BUFFER_READF32,
                    udata1,
                    off_x,
                    tag_udata,
                );
                let y = build.inst_ir_cmd_ir_op_ir_op_ir_op(
                    IrCmd::BUFFER_READF32,
                    udata1,
                    off_y,
                    tag_udata,
                );

                let mx = build.inst_ir_cmd_ir_op(IrCmd::UNM_FLOAT, x);
                let my = build.inst_ir_cmd_ir_op(IrCmd::UNM_FLOAT, y);

                build.inst_ir_cmd(IrCmd::CHECK_GC);
                let size = build.const_int(core::mem::size_of::<Vec2>() as i32);
                let tag_vec2 = build.const_int(K_TAG_VEC2);
                let udatar = build.inst_ir_cmd_ir_op_ir_op(IrCmd::NEW_USERDATA, size, tag_vec2);

                let tag_udata = build.const_tag(LUA_TUSERDATA);
                build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
                    IrCmd::BUFFER_WRITEF32,
                    udatar,
                    off_x,
                    mx,
                    tag_udata,
                );
                build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
                    IrCmd::BUFFER_WRITEF32,
                    udatar,
                    off_y,
                    my,
                    tag_udata,
                );

                let vm_reg = build.vm_reg(result_reg as u8);
                build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_POINTER, vm_reg, udatar);
                let tag_udata = build.const_tag(LUA_TUSERDATA);
                build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, vm_reg, tag_udata);

                return true;
            }
        }
        _ => {}
    }
    false
}
