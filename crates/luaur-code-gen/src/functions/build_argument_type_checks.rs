use crate::enums::ir_block_kind::IrBlockKind;
use crate::enums::ir_cmd::IrCmd;
use crate::functions::has_typed_parameters::has_typed_parameters;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_common::enums::luau_bytecode_type::{
    LBC_TYPE_ANY, LBC_TYPE_BOOLEAN, LBC_TYPE_BUFFER, LBC_TYPE_FUNCTION, LBC_TYPE_INTEGER,
    LBC_TYPE_NIL, LBC_TYPE_NUMBER, LBC_TYPE_OPTIONAL_BIT, LBC_TYPE_STRING, LBC_TYPE_TABLE,
    LBC_TYPE_TAGGED_USERDATA_BASE, LBC_TYPE_TAGGED_USERDATA_END, LBC_TYPE_THREAD,
    LBC_TYPE_USERDATA, LBC_TYPE_VECTOR,
};
use luaur_vm::enums::lua_type::lua_Type;

const K_BLOCK_FLAG_ENTRY_ARG_CHECK: u8 = 1 << 2;
const K_VM_EXIT_ENTRY_GUARD_PC: u32 = (1u32 << 28) - 1;

pub fn build_argument_type_checks(build: &mut IrBuilder, entry: IrOp) {
    let argument_types = build.function.bc_original_type_info.argument_types.clone();
    CODEGEN_ASSERT!(has_typed_parameters(&build.function.bc_original_type_info));

    build.function.block_op(entry).flags |= K_BLOCK_FLAG_ENTRY_ARG_CHECK;

    for (i, et) in argument_types.iter().copied().enumerate() {
        let tag = et & !(LBC_TYPE_OPTIONAL_BIT.0 as u8);
        let optional = (et & (LBC_TYPE_OPTIONAL_BIT.0 as u8)) != 0;

        if tag == LBC_TYPE_ANY.0 as u8 {
            continue;
        }

        let reg = build.vm_reg(i as u8);
        let load = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, reg);

        let mut next_check = IrOp::default();
        if optional {
            next_check = build.block(IrBlockKind::Internal);
            let fallback_check = build.block(IrBlockKind::Internal);
            let nil = build.const_tag(lua_Type::LUA_TNIL as u8);
            build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
                IrCmd::JUMP_EQ_TAG,
                load,
                nil,
                next_check,
                fallback_check,
            );

            build.begin_block(fallback_check);
            build.function.block_op(fallback_check).flags |= K_BLOCK_FLAG_ENTRY_ARG_CHECK;
        }

        let luau_tag = match tag {
            x if x == LBC_TYPE_NIL.0 as u8 => lua_Type::LUA_TNIL as u8,
            x if x == LBC_TYPE_BOOLEAN.0 as u8 => lua_Type::LUA_TBOOLEAN as u8,
            x if x == LBC_TYPE_NUMBER.0 as u8 => lua_Type::LUA_TNUMBER as u8,
            x if x == LBC_TYPE_INTEGER.0 as u8 => lua_Type::LUA_TINTEGER as u8,
            x if x == LBC_TYPE_STRING.0 as u8 => lua_Type::LUA_TSTRING as u8,
            x if x == LBC_TYPE_TABLE.0 as u8 => lua_Type::LUA_TTABLE as u8,
            x if x == LBC_TYPE_FUNCTION.0 as u8 => lua_Type::LUA_TFUNCTION as u8,
            x if x == LBC_TYPE_THREAD.0 as u8 => lua_Type::LUA_TTHREAD as u8,
            x if x == LBC_TYPE_USERDATA.0 as u8 => lua_Type::LUA_TUSERDATA as u8,
            x if x == LBC_TYPE_VECTOR.0 as u8 => lua_Type::LUA_TVECTOR as u8,
            x if x == LBC_TYPE_BUFFER.0 as u8 => lua_Type::LUA_TBUFFER as u8,
            x if x >= LBC_TYPE_TAGGED_USERDATA_BASE.0 as u8
                && x < LBC_TYPE_TAGGED_USERDATA_END.0 as u8 =>
            {
                lua_Type::LUA_TUSERDATA as u8
            }
            _ => {
                CODEGEN_ASSERT!(false);
                lua_Type::LUA_TNIL as u8
            }
        };

        let tag_op = build.const_tag(luau_tag);
        let exit = build.vm_exit(K_VM_EXIT_ENTRY_GUARD_PC);
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, load, tag_op, exit);

        if optional {
            build.inst_ir_cmd_ir_op(IrCmd::JUMP, next_check);
            build.begin_block(next_check);
            build.function.block_op(next_check).flags |= K_BLOCK_FLAG_ENTRY_ARG_CHECK;
        }
    }

    if argument_types
        .last()
        .is_some_and(|ty| (ty & (LBC_TYPE_OPTIONAL_BIT.0 as u8)) == 0)
    {
        let next = build.block(IrBlockKind::Internal);
        build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
        build.begin_block(next);
    }
}
