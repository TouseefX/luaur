use crate::enums::size_x_64::SizeX64;
use crate::records::ir_lowering_x_64::IrLoweringX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;
use crate::records::scoped_reg_x_64::ScopedRegX64;

impl IrLoweringX64 {
    pub fn increment_counter_at(&mut self, offset: usize) {
        unsafe {
            let mut tmp = ScopedRegX64 {
                owner: &mut self.regs,
                reg: RegisterX64::noreg,
            };
            tmp.alloc(SizeX64::qword);

            // Get counter slot
            (*self.build).mov(OperandX64::reg(tmp.reg), OperandX64::reg(RegisterX64::rdi));
            (*self.build).mov(
                OperandX64::reg(tmp.reg),
                OperandX64::mem(
                    SizeX64::qword,
                    RegisterX64::noreg,
                    1,
                    tmp.reg,
                    (core::mem::offset_of!(luaur_vm::records::closure::Closure, inner)
                        + core::mem::offset_of!(luaur_vm::records::closure::LClosure, p))
                        as i32,
                ),
            );
            (*self.build).mov(
                OperandX64::reg(tmp.reg),
                OperandX64::mem(
                    SizeX64::qword,
                    RegisterX64::noreg,
                    1,
                    tmp.reg,
                    core::mem::offset_of!(luaur_vm::records::proto::Proto, execdata) as i32,
                ),
            );

            // Increment
            (*self.build).inc(OperandX64::mem(
                SizeX64::qword,
                RegisterX64::noreg,
                1,
                tmp.reg,
                (((*(*self.function).proto).sizecode as u32 + offset as u32) * 4) as i32,
            ));
        }
    }
}
