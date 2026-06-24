use alloc::boxed::Box;
use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
use luaur_code_gen::records::ir_function::IrFunction;
use luaur_code_gen::records::ir_reg_alloc_x_64::IrRegAllocX64;

#[derive(Debug)]
pub struct IrRegAllocX64Fixture {
    pub(crate) build: Box<AssemblyBuilderX64>,
    pub(crate) function: Box<IrFunction>,
    pub(crate) regs: IrRegAllocX64,
}
