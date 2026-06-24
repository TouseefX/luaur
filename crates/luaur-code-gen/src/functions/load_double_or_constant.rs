use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::vm_const_op::vm_const_op;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_vm::type_aliases::t_value::TValue;

pub fn load_double_or_constant(build: &mut IrBuilder, arg: IrOp) -> IrOp {
    if arg.kind() == IrOpKind::VmConst {
        CODEGEN_ASSERT!(!build.function.proto.is_null());
        let protok_idx = vm_const_op(arg) as usize;
        let protok: TValue = unsafe { *(*build.function.proto).k.add(protok_idx) };
        CODEGEN_ASSERT!(protok.tt == luaur_vm::enums::lua_type::lua_Type::LUA_TNUMBER as i32);
        return build.const_double(unsafe { protok.value.n });
    }

    build.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, arg)
}
