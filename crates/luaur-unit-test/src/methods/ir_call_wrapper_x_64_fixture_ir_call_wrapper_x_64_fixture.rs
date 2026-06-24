//! Generated skeleton item.
//! Node: `cxx:Method:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:13:ir_call_wrapper_x_64_fixture_ir_call_wrapper_x_64_fixture`
//! Source: `tests/IrCallWrapperX64.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
//! - source_includes:
//!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
//!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
//! - incoming:
//!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
//!   - type_ref <- record IrCallWrapperX64Fixture (tests/IrCallWrapperX64.test.cpp)
//! - outgoing:
//!   - type_ref -> enum ABIX64 (CodeGen/include/Luau/AssemblyBuilderX64.h)
//!   - type_ref -> record IrCallWrapperX64Fixture (tests/IrCallWrapperX64.test.cpp)
//!   - translates_to -> rust_item IrCallWrapperX64Fixture::IrCallWrapperX64Fixture

use crate::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;
use alloc::boxed::Box;
use luaur_code_gen::enums::abix_64::ABIX64;
use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
use luaur_code_gen::records::ir_call_wrapper_x_64::IrCallWrapperX64;
use luaur_code_gen::records::ir_data::k_invalid_inst_idx;
use luaur_code_gen::records::ir_function::IrFunction;
use luaur_code_gen::records::ir_op::IrOp;
use luaur_code_gen::records::ir_reg_alloc_x_64::IrRegAllocX64;
use luaur_code_gen::records::operand_x_64::OperandX64;
use luaur_code_gen::records::register_x_64::RegisterX64;
use luaur_code_gen::records::scoped_reg_x_64::ScopedRegX64;

impl IrCallWrapperX64Fixture {
    pub fn new(abi: ABIX64) -> Self {
        let mut build = Box::new(AssemblyBuilderX64::assembly_builder_x_64_bool_abix_64_i32(
            true, abi, 0,
        ));
        let mut function = Box::new(IrFunction::default());
        let mut regs = Box::new(IrRegAllocX64::ir_reg_alloc_x_64_ir_reg_alloc_x_64(
            &mut build,
            &mut function,
            core::ptr::null_mut(),
        ));
        let call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
            &mut regs, &mut build, !0u32,
        );

        Self {
            build,
            function,
            regs,
            call_wrap,
            r_arg1: RegisterX64::rcx,
            r_arg1d: RegisterX64::ecx,
            r_arg2: RegisterX64::rdx,
            r_arg2d: RegisterX64::edx,
            r_arg3: RegisterX64::r8,
            r_arg3d: RegisterX64::r8d,
            r_arg4: RegisterX64::r9,
            r_arg4d: RegisterX64::r9d,
        }
    }

    pub fn windows() -> Self {
        Self::new(ABIX64::Windows)
    }

    pub fn take_scoped(&mut self, reg: RegisterX64) -> ScopedRegX64 {
        let reg = self.regs.take_reg(reg, k_invalid_inst_idx);
        ScopedRegX64 {
            owner: &mut *self.regs,
            reg,
        }
    }

    pub fn add_arg(
        &mut self,
        target_size: luaur_code_gen::enums::size_x_64::SizeX64,
        source: impl Into<OperandX64>,
    ) {
        self.call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            target_size,
            source.into(),
            IrOp::ir_op(),
        );
    }

    pub fn add_scoped(
        &mut self,
        target_size: luaur_code_gen::enums::size_x_64::SizeX64,
        scoped_reg: &mut ScopedRegX64,
    ) {
        self.call_wrap
            .add_argument_size_x_64_scoped_reg_x_64(target_size, scoped_reg);
    }

    pub fn call(&mut self, func: OperandX64) {
        self.call_wrap.call(&func);
    }
}

pub fn ir_call_wrapper_x_64_fixture_ir_call_wrapper_x_64_fixture() -> IrCallWrapperX64Fixture {
    IrCallWrapperX64Fixture::windows()
}
