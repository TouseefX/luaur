use crate::enums::builtin_impl_type::BuiltinImplType;
use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_const_kind::IrConstKind;
use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::vm_const_op::vm_const_op;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::builtin_impl_result::BuiltinImplResult;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_builtin_table_insert(
    build: &mut IrBuilder,
    nparams: i32,
    ra: i32,
    arg: i32,
    args: IrOp,
    nresults: i32,
    pcpos: i32,
) -> BuiltinImplResult {
    if nparams != 2 || nresults > 0 {
        return BuiltinImplResult {
            r#type: BuiltinImplType::None,
            actual_result_count: -1,
        };
    }

    let arg_reg = build.vm_reg(arg as u8);
    let exit = build.vm_exit(pcpos as u32);
    build.load_and_check_tag(arg_reg, lua_Type::LUA_TTABLE as u8, exit);

    let arg_reg = build.vm_reg(arg as u8);
    let table = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, arg_reg);
    let exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::CHECK_READONLY, table, exit);

    let len = build.inst_ir_cmd_ir_op(IrCmd::TABLE_LEN, table);
    let one = build.const_int(1);
    let pos = build.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_INT, len, one);
    let setnum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::TABLE_SETNUM, table, pos);

    if args.kind() == IrOpKind::Constant {
        CODEGEN_ASSERT!(build.function.const_op(args).kind == IrConstKind::Double);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, setnum, args);
        let tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, setnum, tag);
    } else {
        let va = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, args);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, setnum, va);

        CODEGEN_ASSERT!(!build.function.proto.is_null());
        let argstag = if args.kind() == IrOpKind::VmConst {
            let tag = unsafe {
                (*build.function.proto)
                    .k
                    .add(vm_const_op(args) as usize)
                    .read()
                    .tt
            };
            build.const_tag(tag as u8)
        } else {
            build.undef()
        };
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BARRIER_TABLE_FORWARD, table, args, argstag);
    }

    BuiltinImplResult {
        r#type: BuiltinImplType::Full,
        actual_result_count: 0,
    }
}
