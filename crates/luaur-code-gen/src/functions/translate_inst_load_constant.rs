use crate::enums::ir_cmd::IrCmd;
use crate::records::ir_builder::IrBuilder;
use luaur_common::FFlag;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::type_aliases::t_value::TValue;

pub fn translate_inst_load_constant(build: &mut IrBuilder, ra: i32, k: i32) {
    let proto = unsafe { &(*build.function.proto) };
    let protok = unsafe { *proto.k.add(k as usize) } as TValue;

    if protok.tt == lua_Type::LUA_TNIL as i32 {
        let ra_reg = build.vm_reg(ra as u8);
        let tag = build.const_tag(lua_Type::LUA_TNIL as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_reg, tag);
    } else if protok.tt == lua_Type::LUA_TBOOLEAN as i32 {
        let ra_reg = build.vm_reg(ra as u8);
        let value = build.const_int(unsafe { protok.value.b } as i32);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, ra_reg, value);
        let tag = build.const_tag(lua_Type::LUA_TBOOLEAN as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_reg, tag);
    } else if FFlag::LuauCodegenInteger2.get() && protok.tt == lua_Type::LUA_TINTEGER as i32 {
        let ra_reg = build.vm_reg(ra as u8);
        let value = build.const_int_64(unsafe { protok.value.l });
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, ra_reg, value);
        let tag = build.const_tag(lua_Type::LUA_TINTEGER as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_reg, tag);
    } else if protok.tt == lua_Type::LUA_TNUMBER as i32 {
        let ra_reg = build.vm_reg(ra as u8);
        let value = build.const_double(unsafe { protok.value.n });
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, ra_reg, value);
        let tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_reg, tag);
    } else {
        let const_op = build.vm_const(k as u32);
        let offset = build.const_int(0);
        let tag = build.const_tag(protok.tt as u8);
        let load = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::LOAD_TVALUE, const_op, offset, tag);
        let ra_reg = build.vm_reg(ra as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, ra_reg, load);
    }
}
