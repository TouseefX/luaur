use crate::enums::ir_const_kind::IrConstKind;
use crate::enums::ir_op_kind::IrOpKind;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_vm::enums::lua_type::lua_Type;

pub fn builtin_check_double(build: &mut IrBuilder, arg: IrOp, pcpos: i32) {
    if arg.kind() == IrOpKind::Constant {
        CODEGEN_ASSERT!(build.function.const_op(arg).kind == IrConstKind::Double);
    } else {
        let exit = build.vm_exit(pcpos as u32);
        build.load_and_check_tag(arg, lua_Type::LUA_TNUMBER as u8, exit);
    }
}
