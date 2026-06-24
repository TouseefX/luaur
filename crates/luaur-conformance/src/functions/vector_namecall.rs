use crate::functions::compare_member_name::compare_member_name;
use luaur_code_gen::enums::ir_cmd::IrCmd;
use luaur_code_gen::records::ir_builder::IrBuilder;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::macros::lua_multret::LUA_MULTRET;

pub fn vector_namecall(
    build: &mut IrBuilder,
    member: *const core::ffi::c_char,
    member_length: usize,
    arg_res_reg: i32,
    source_reg: i32,
    params: i32,
    results: i32,
    pcpos: i32,
) -> bool {
    if compare_member_name(member, member_length, c"Dot".as_ptr()) && params == 2 && results <= 1 {
        let arg_reg = build.vm_reg((arg_res_reg + 2) as u8);
        let exit = build.vm_exit(pcpos as u32);
        build.load_and_check_tag(arg_reg, lua_Type::LUA_TVECTOR as u8, exit);

        let src_reg = build.vm_reg(source_reg as u8);
        let arg_reg = build.vm_reg((arg_res_reg + 2) as u8);
        let c0 = build.const_int(0);
        let c4 = build.const_int(4);
        let c8 = build.const_int(8);

        let x1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, src_reg, c0);
        let x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, arg_reg, c0);
        let xx = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, x1, x2);

        let y1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, src_reg, c4);
        let y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, arg_reg, c4);
        let yy = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, y1, y2);

        let z1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, src_reg, c8);
        let z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, arg_reg, c8);
        let zz = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, z1, z2);

        let sum_xy = build.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_FLOAT, xx, yy);
        let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_FLOAT, sum_xy, zz);

        let res_reg = build.vm_reg(arg_res_reg as u8);
        let num = build.inst_ir_cmd_ir_op(IrCmd::FLOAT_TO_NUM, sum);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, res_reg, num);
        let tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, res_reg, tag);

        if results == LUA_MULTRET {
            let adj_arg = build.vm_reg(arg_res_reg as u8);
            let adj_val = build.const_int(1);
            build.inst_ir_cmd_ir_op_ir_op(IrCmd::ADJUST_STACK_TO_REG, adj_arg, adj_val);
        }

        return true;
    }

    if compare_member_name(member, member_length, c"Cross".as_ptr()) && params == 2 && results <= 1
    {
        let arg_reg = build.vm_reg((arg_res_reg + 2) as u8);
        let exit = build.vm_exit(pcpos as u32);
        build.load_and_check_tag(arg_reg, lua_Type::LUA_TVECTOR as u8, exit);

        let src_reg = build.vm_reg(source_reg as u8);
        let arg_reg = build.vm_reg((arg_res_reg + 2) as u8);
        let c0 = build.const_int(0);
        let c4 = build.const_int(4);
        let c8 = build.const_int(8);

        let x1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, src_reg, c0);
        let x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, arg_reg, c0);
        let y1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, src_reg, c4);
        let y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, arg_reg, c4);
        let z1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, src_reg, c8);
        let z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, arg_reg, c8);

        let y1z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, y1, z2);
        let z1y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, z1, y2);
        let xr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SUB_FLOAT, y1z2, z1y2);

        let z1x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, z1, x2);
        let x1z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, x1, z2);
        let yr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SUB_FLOAT, z1x2, x1z2);

        let x1y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, x1, y2);
        let y1x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_FLOAT, y1, x2);
        let zr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SUB_FLOAT, x1y2, y1x2);

        let res_reg = build.vm_reg(arg_res_reg as u8);
        build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::STORE_VECTOR, res_reg, xr, yr, zr);
        let tag = build.const_tag(lua_Type::LUA_TVECTOR as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, res_reg, tag);

        if results == LUA_MULTRET {
            let adj_arg = build.vm_reg(arg_res_reg as u8);
            let adj_val = build.const_int(1);
            build.inst_ir_cmd_ir_op_ir_op(IrCmd::ADJUST_STACK_TO_REG, adj_arg, adj_val);
        }

        return true;
    }

    false
}
