use alloc::boxed::Box;

use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
use luaur_code_gen::records::ir_call_wrapper_x_64::IrCallWrapperX64;
use luaur_code_gen::records::ir_function::IrFunction;
use luaur_code_gen::records::ir_reg_alloc_x_64::IrRegAllocX64;
use luaur_code_gen::records::register_x_64::RegisterX64;

#[derive(Debug)]
pub struct IrCallWrapperX64Fixture {
    pub(crate) build: Box<AssemblyBuilderX64>,
    pub(crate) function: Box<IrFunction>,
    pub(crate) regs: Box<IrRegAllocX64>,
    pub(crate) call_wrap: IrCallWrapperX64,

    // Tests rely on these to force interference between registers
    pub(crate) r_arg1: RegisterX64,
    pub(crate) r_arg1d: RegisterX64,
    pub(crate) r_arg2: RegisterX64,
    pub(crate) r_arg2d: RegisterX64,
    pub(crate) r_arg3: RegisterX64,
    pub(crate) r_arg3d: RegisterX64,
    pub(crate) r_arg4: RegisterX64,
    pub(crate) r_arg4d: RegisterX64,
}
