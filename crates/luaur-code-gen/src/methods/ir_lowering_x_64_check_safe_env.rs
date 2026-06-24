use crate::enums::condition_x_64::ConditionX64;
use crate::enums::size_x_64::SizeX64;
use crate::records::ir_block::IrBlock;
use crate::records::ir_lowering_x_64::IrLoweringX64;
use crate::records::ir_op::IrOp;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;
use crate::records::scoped_reg_x_64::ScopedRegX64;
use luaur_vm::records::closure::Closure;
use luaur_vm::type_aliases::lua_table::LuaTable;

const K_STACK_OFFSET_TO_LOCALS: i32 = 16 + 32;

fn s_closure() -> OperandX64 {
    OperandX64::mem(
        SizeX64::qword,
        RegisterX64::noreg,
        1,
        RegisterX64::rsp,
        K_STACK_OFFSET_TO_LOCALS,
    )
}

fn mem(size: SizeX64, base: RegisterX64, disp: i32) -> OperandX64 {
    OperandX64::mem(size, RegisterX64::noreg, 1, base, disp)
}

impl IrLoweringX64 {
    pub fn check_safe_env(&mut self, target: IrOp, index: u32, next: &IrBlock) {
        unsafe {
            let mut tmp = ScopedRegX64 {
                owner: &mut self.regs,
                reg: RegisterX64::noreg,
            };
            tmp.alloc(SizeX64::qword);

            (*self.build).mov(OperandX64::reg(tmp.reg), s_closure());
            (*self.build).mov(
                OperandX64::reg(tmp.reg),
                mem(
                    SizeX64::qword,
                    tmp.reg,
                    core::mem::offset_of!(Closure, env) as i32,
                ),
            );
            (*self.build).cmp(
                mem(
                    SizeX64::byte,
                    tmp.reg,
                    core::mem::offset_of!(LuaTable, safeenv) as i32,
                ),
                OperandX64::imm(0),
            );
        }

        self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
            ConditionX64::Equal,
            target,
            index,
            next,
        );
    }
}
